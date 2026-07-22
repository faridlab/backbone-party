-- Migration: tenant-scope party (ADR-0010 Decision B1)
-- Hand-authored (user-owned). Not regenerated.
--
-- Party (customers/suppliers/contacts/addresses/emails/phones) was fully global — zero
-- company_id, zero RLS. This migration makes every party entity tenant-scoped and fences
-- each table with the ADR-0008 RLS invariant:
--
--     company_id = NULLIF(current_setting('app.company_id', true), '')::uuid
--
-- Per-table shape: ADD COLUMN company_id UUID → backfill → SET NOT NULL →
-- ENABLE + FORCE RLS → POLICY. Global unique indexes (party_code, npwp, nik on parties;
-- party_id primary-per-party on each child) are replaced with per-company composite uniques,
-- preserving the `deleted_at IS NULL` soft-delete WHERE.
--
-- BACKFILL + FENCE POLICY (ADR-0010 B1, mirrors catalog's final pattern):
--   - If `organization.companies` has exactly one live row, backfill every party row + every
--     child row to it (convenience for the single-company / dev / demo case). Children are
--     backfilled to their parent party's company so the fence lands on consistent rows.
--   - The RLS fence (NOT NULL + ENABLE + FORCE + POLICY) is then armed UNCONDITIONALLY on
--     every table that has zero NULL company_id rows.
--   - If ANY party row still has NULL company_id after backfill (the multi-company or
--     no-organization case with existing data), the migration FAILS LOUD — RAISE EXCEPTION
--     naming every stray table + row count — rather than silently leaving the fence disarmed.
--     The operator must assign those rows (or confirm a fresh DB), then re-run; the migration
--     is idempotent and will arm the fence once clean. We never pick an arbitrary company_id,
--     and we never ship a disarmed fence in the multi-tenant case where it is needed most.
--
-- No SQL FK to organization.companies is added: party is a framework module and must stay
-- independently deployable. RLS is the fence, not the FK (matches catalog / pos).

-- ==============================================================================
-- Step 1: ADD COLUMN company_id UUID (nullable) on every party table.
-- ==============================================================================

ALTER TABLE party.parties         ADD COLUMN IF NOT EXISTS company_id UUID;
ALTER TABLE party.party_addresses ADD COLUMN IF NOT EXISTS company_id UUID;
ALTER TABLE party.party_contacts  ADD COLUMN IF NOT EXISTS company_id UUID;
ALTER TABLE party.party_emails    ADD COLUMN IF NOT EXISTS company_id UUID;
ALTER TABLE party.party_phones    ADD COLUMN IF NOT EXISTS company_id UUID;

-- Supporting index for per-company queries (added unconditionally; cheap).
CREATE INDEX IF NOT EXISTS idx_parties_company_id          ON party.parties (company_id);
CREATE INDEX IF NOT EXISTS idx_party_addresses_company_id  ON party.party_addresses (company_id);
CREATE INDEX IF NOT EXISTS idx_party_contacts_company_id   ON party.party_contacts (company_id);
CREATE INDEX IF NOT EXISTS idx_party_emails_company_id     ON party.party_emails (company_id);
CREATE INDEX IF NOT EXISTS idx_party_phones_company_id     ON party.party_phones (company_id);

-- ===============================================================================
-- Step 2: BACKFILL — only when exactly one live company exists (convenience).
-- Multi-company / no-org deployments skip backfill; Step 4 then fails loud on any
-- remaining NULL rows so the fence is never silently disarmed.
-- Children are backfilled from their parent party's company after parties are filled,
-- so a pre-existing single-company DB lands with every row consistent.
-- ===============================================================================

DO $$
DECLARE
    has_org boolean;
    cnt     int;
    cid     uuid;
BEGIN
    SELECT EXISTS (SELECT 1 FROM pg_namespace WHERE nspname = 'organization') INTO has_org;
    IF has_org THEN
        EXECUTE $q$
            SELECT COUNT(*) FROM organization.companies
            WHERE (metadata ->> 'deleted_at') IS NULL
        $q$ INTO cnt;
    ELSE
        -- organization schema not installed in this deployment → unresolvable.
        cnt := -1;
    END IF;

    IF cnt = 1 THEN
        EXECUTE $q$
            SELECT id FROM organization.companies
            WHERE (metadata ->> 'deleted_at') IS NULL
            LIMIT 1
        $q$ INTO cid;

        RAISE NOTICE 'party ADR-0010 B1: exactly 1 company (%) — backfilling parties + children', cid;
        -- Root: parties first.
        UPDATE party.parties SET company_id = cid WHERE company_id IS NULL;
        -- Children: inherit from their parent party (handles pre-existing rows regardless
        -- of which company the parent resolved to — robust if party.company_id was partly
        -- populated by an earlier partial run).
        UPDATE party.party_addresses pa
            SET company_id = p.company_id
            FROM party.parties p
            WHERE pa.party_id = p.id AND pa.company_id IS NULL;
        UPDATE party.party_contacts pc
            SET company_id = p.company_id
            FROM party.parties p
            WHERE pc.party_id = p.id AND pc.company_id IS NULL;
        UPDATE party.party_emails pe
            SET company_id = p.company_id
            FROM party.parties p
            WHERE pe.party_id = p.id AND pe.company_id IS NULL;
        UPDATE party.party_phones pp
            SET company_id = p.company_id
            FROM party.parties p
            WHERE pp.party_id = p.id AND pp.company_id IS NULL;
    ELSE
        -- AMBIGUOUS (0 or >1 companies, or no organization schema). Do NOT backfill and
        -- do NOT pick an arbitrary company. Step 4 will fail loud if any NULL rows remain.
        RAISE NOTICE
            'party ADR-0010 B1: backfill skipped (organization.companies live-row count=%). '
            'Step 4 will fail loud on any party rows still missing company_id.',
            cnt;
    END IF;
END $$;

-- ===============================================================================
-- Step 3: UNIQUE INDEX change — drop global, create per-company composite.
-- (Always applied: the column is nullable-safe with the partial WHERE, and once
--  backfill is resolved later these indexes must already be per-company.)
-- ===============================================================================

-- ── parties: party_code, npwp, nik ─────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_parties_party_code;
DROP INDEX IF EXISTS party.idx_parties_npwp;
DROP INDEX IF EXISTS party.idx_parties_nik;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_company_id_party_code
    ON party.parties (company_id, party_code)
    WHERE (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_company_id_npwp
    ON party.parties (company_id, npwp)
    WHERE npwp IS NOT NULL AND (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_company_id_nik
    ON party.parties (company_id, nik)
    WHERE nik IS NOT NULL AND (metadata ->> 'deleted_at') IS NULL;

-- Also rebuild the former secondary indexes to lead with company_id (matches the new
-- per-company query shape; the original non-company variants are dropped to avoid
-- cross-tenant index sprawl).
DROP INDEX IF EXISTS party.idx_parties_party_kind_status;
DROP INDEX IF EXISTS party.idx_parties_name;
CREATE INDEX IF NOT EXISTS idx_parties_company_id_kind_status
    ON party.parties (company_id, party_kind, status);
CREATE INDEX IF NOT EXISTS idx_parties_company_id_name
    ON party.parties (company_id, name);

-- ── party_addresses: one primary per party ─────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_addresses_party_id;
DROP INDEX IF EXISTS party.idx_party_addresses_party_id_is_primary;
DROP INDEX IF EXISTS party.idx_party_addresses_party_id_address_type;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_addresses_company_id_party_id_primary
    ON party.party_addresses (company_id, party_id)
    WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_addresses_company_id_party_id_is_primary
    ON party.party_addresses (company_id, party_id, is_primary);
CREATE INDEX IF NOT EXISTS idx_party_addresses_company_id_party_id_address_type
    ON party.party_addresses (company_id, party_id, address_type);

-- ── party_contacts: one primary per party ──────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_contacts_party_id;
DROP INDEX IF EXISTS party.idx_party_contacts_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_contacts_company_id_party_id_primary
    ON party.party_contacts (company_id, party_id)
    WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_contacts_company_id_party_id_is_primary
    ON party.party_contacts (company_id, party_id, is_primary);

-- ── party_emails: one primary per party ────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_emails_party_id;
DROP INDEX IF EXISTS party.idx_party_emails_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_emails_company_id_party_id_primary
    ON party.party_emails (company_id, party_id)
    WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_emails_company_id_party_id_is_primary
    ON party.party_emails (company_id, party_id, is_primary);

-- ── party_phones: one primary per party ────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_phones_party_id;
DROP INDEX IF EXISTS party.idx_party_phones_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_phones_company_id_party_id_primary
    ON party.party_phones (company_id, party_id)
    WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_phones_company_id_party_id_is_primary
    ON party.party_phones (company_id, party_id, is_primary);

-- ===============================================================================
-- Step 4: FAIL LOUD on strays, then arm the fence on ALL tables atomically.
-- First sweep counts NULL-company_id rows per table. If ANY exist, RAISE EXCEPTION
-- listing every stray table + count and abort — the fence is never partially armed
-- and never silently disarmed in the multi-tenant case. If all clean, arm all 5.
-- Idempotent: after the operator assigns strays, re-running arms the fence.
-- ===============================================================================

DO $$
DECLARE
    t            text;
    null_rows    int;
    tabs         text[] := ARRAY[
        'parties','party_addresses','party_contacts','party_emails','party_phones'
    ];
    strays       text[] := ARRAY[]::text[];
BEGIN
    -- Sweep: collect every table that still has unassigned rows.
    FOREACH t IN ARRAY tabs LOOP
        EXECUTE format('SELECT COUNT(*) FROM party.%I WHERE company_id IS NULL', t) INTO null_rows;
        IF null_rows > 0 THEN
            strays := array_append(strays, format('%I=%s', t, null_rows));
        END IF;
    END LOOP;

    -- Fail loud if anything is unresolved — do NOT ship a disarmed fence.
    IF array_length(strays, 1) IS NOT NULL THEN
        RAISE EXCEPTION
            'party ADR-0010 B1: refusing to fence — % party table(s) still have NULL company_id (%). '
            'Assign every party row to a tenant (or confirm a fresh DB), then re-run this migration. '
            'No RLS fence has been armed.',
            array_length(strays, 1), array_to_string(strays, ', ');
    END IF;

    -- All clean → arm the fence on every table.
    FOREACH t IN ARRAY tabs LOOP
        EXECUTE format('ALTER TABLE party.%I ALTER COLUMN company_id SET NOT NULL', t);
        EXECUTE format('ALTER TABLE party.%I ENABLE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE party.%I FORCE  ROW LEVEL SECURITY', t);
        EXECUTE format(
            'DROP POLICY IF EXISTS %I ON party.%I; '
            'CREATE POLICY %I ON party.%I FOR ALL '
            'USING      (company_id = NULLIF(current_setting(''app.company_id'', true), '''')::uuid) '
            'WITH CHECK (company_id = NULLIF(current_setting(''app.company_id'', true), '''')::uuid)',
            t || '_company_isolation', t,
            t || '_company_isolation', t
        );
    END LOOP;

    RAISE NOTICE 'party ADR-0010 B1: RLS fence live on all 5 party tables.';
END $$;
