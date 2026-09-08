-- Hand-authored (user-owned). Not regenerated.
--
-- Strip every company-fence artifact from the party tables (ADR-0029): the module is
-- tenant-agnostic; org scoping is installed by the COMPOSING service's tenancy decorator,
-- never by the module. Dropped here, per table: the company-leading indexes, the
-- <table>_company_isolation RLS policy, and the company_id column itself.
--
-- Ordering guard (the decorator must run FIRST on any database with data): the module
-- never moves tenancy data. A table is safe to strip when EITHER
--   a) it carries org_unit_id with no NULLs — the decorator backfilled it from company_id —
--      or b) it is empty (a fresh database: the earlier chain files created it empty).
-- Otherwise the strip RAISEs, naming the decorator step, rather than dropping a column
-- that still holds the only tenancy key. The file is re-runnable (every drop is IF EXISTS
-- and the tracker has no checksums), so a failed run retries cleanly after the decorator
-- lands.
--
-- RLS enable/force flags are deliberately NOT touched: the decorator owns those now.

DO $$
DECLARE
    t text;
    has_org boolean;
    org_nulls bigint;
    total bigint;
    offenders text := '';
BEGIN
    FOREACH t IN ARRAY ARRAY['parties', 'party_addresses', 'party_contacts', 'party_emails', 'party_phones']
    LOOP
        IF to_regclass(format('party.%I', t)) IS NULL THEN
            CONTINUE; -- chain not fully applied on this database; nothing to strip
        END IF;

        SELECT EXISTS (
                   SELECT 1 FROM information_schema.columns
                   WHERE table_schema = 'party' AND table_name = t AND column_name = 'org_unit_id'
               )
        INTO has_org;

        EXECUTE format('SELECT count(*) FROM party.%I', t) INTO total;

        IF has_org THEN
            EXECUTE format(
                'SELECT count(*) FROM party.%I WHERE org_unit_id IS NULL', t)
            INTO org_nulls;
        ELSE
            org_nulls := total; -- no org column: every row's only tenancy key is company_id
        END IF;

        IF has_org AND org_nulls = 0 THEN
            CONTINUE; -- decorator backfilled: safe
        END IF;
        IF total = 0 THEN
            CONTINUE; -- empty table (fresh database): safe
        END IF;
        offenders := offenders || format(' party.%s (%s rows, %s rows not covered by org_unit_id);', t, total, org_nulls);
    END LOOP;

    IF offenders <> '' THEN
        RAISE EXCEPTION 'refusing to strip company_id — these tables are not yet covered by the tenancy decorator:%. Apply the composing service''s tenancy decorator (it backfills org_unit_id from company_id) and re-run; it is the only step that moves tenancy data.', offenders;
    END IF;
END $$;

-- ── parties ────────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_parties_company_id;
DROP INDEX IF EXISTS party.idx_parties_company_id_party_code;
DROP INDEX IF EXISTS party.idx_parties_company_id_npwp;
DROP INDEX IF EXISTS party.idx_parties_company_id_nik;
DROP INDEX IF EXISTS party.idx_parties_company_id_kind_status;
DROP INDEX IF EXISTS party.idx_parties_company_id_name;
DROP POLICY IF EXISTS parties_company_isolation ON party.parties;
ALTER TABLE party.parties DROP COLUMN IF EXISTS company_id;

-- ── party_addresses ────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_addresses_company_id;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_is_primary;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_address_type;
DROP POLICY IF EXISTS party_addresses_company_isolation ON party.party_addresses;
ALTER TABLE party.party_addresses DROP COLUMN IF EXISTS company_id;

-- ── party_contacts ─────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_contacts_company_id;
DROP INDEX IF EXISTS party.idx_party_contacts_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_contacts_company_id_party_id_is_primary;
DROP POLICY IF EXISTS party_contacts_company_isolation ON party.party_contacts;
ALTER TABLE party.party_contacts DROP COLUMN IF EXISTS company_id;

-- ── party_emails ───────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_emails_company_id;
DROP INDEX IF EXISTS party.idx_party_emails_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_emails_company_id_party_id_is_primary;
DROP POLICY IF EXISTS party_emails_company_isolation ON party.party_emails;
ALTER TABLE party.party_emails DROP COLUMN IF EXISTS company_id;

-- ── party_phones ───────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_phones_company_id;
DROP INDEX IF EXISTS party.idx_party_phones_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_phones_company_id_party_id_is_primary;
DROP POLICY IF EXISTS party_phones_company_isolation ON party.party_phones;
ALTER TABLE party.party_phones DROP COLUMN IF EXISTS company_id;

-- ── Restore the domain one-primary uniques (tenant-free) ───────────────────────
-- One-primary-per-party is a DOMAIN invariant, not a tenancy posture: a party is one
-- row (id is the PK) living in exactly one unit under any deployment, so the per-party
-- unique needs no tenant column. These carry the exact pre-fence names and predicates.
-- The per-unit uniques on parties (party_code / npwp / nik) are POSTURE and are owned
-- by the composing service's tenancy decorator — they are intentionally NOT restored
-- here (the pre-fence global forms would forbid two units of one tenant sharing a code).
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_addresses_party_id
    ON party.party_addresses (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_contacts_party_id
    ON party.party_contacts (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_emails_party_id
    ON party.party_emails (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_phones_party_id
    ON party.party_phones (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
