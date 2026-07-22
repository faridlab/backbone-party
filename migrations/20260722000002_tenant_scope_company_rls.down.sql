-- Down migration: revert tenant-scope party (ADR-0010 Decision B1).
-- Best-effort. Restores global unique indexes; drops the RLS fence and company_id.
-- WARNING: rows belonging to >1 company cannot be re-collapsed to a single global
-- unique index without conflicts — only run this down if you are sure every
-- company shares globally-unique codes, or after deduplicating.

-- Drop RLS fence + policy on every table (no-op if never armed).
DO $$
DECLARE
    t text;
    tabs text[] := ARRAY[
        'parties','party_addresses','party_contacts','party_emails','party_phones'
    ];
BEGIN
    FOREACH t IN ARRAY tabs LOOP
        EXECUTE format('DROP POLICY IF EXISTS %I ON party.%I', t || '_company_isolation', t);
        EXECUTE format('ALTER TABLE party.%I NO FORCE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE party.%I DISABLE ROW LEVEL SECURITY', t);
    END LOOP;
END $$;

-- Drop composite per-company unique indexes, restore global uniques + original secondary shape.

-- ── parties ────────────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_parties_company_id_party_code;
DROP INDEX IF EXISTS party.idx_parties_company_id_npwp;
DROP INDEX IF EXISTS party.idx_parties_company_id_nik;
DROP INDEX IF EXISTS party.idx_parties_company_id_kind_status;
DROP INDEX IF EXISTS party.idx_parties_company_id_name;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_party_code
    ON party.parties (party_code) WHERE (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_npwp
    ON party.parties (npwp) WHERE npwp IS NOT NULL AND (metadata ->> 'deleted_at') IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_parties_nik
    ON party.parties (nik) WHERE nik IS NOT NULL AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_parties_party_kind_status ON party.parties (party_kind, status);
CREATE INDEX IF NOT EXISTS idx_parties_name ON party.parties (name);

-- ── party_addresses ────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_is_primary;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id_party_id_address_type;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_addresses_party_id
    ON party.party_addresses (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_addresses_party_id_is_primary
    ON party.party_addresses (party_id, is_primary);
CREATE INDEX IF NOT EXISTS idx_party_addresses_party_id_address_type
    ON party.party_addresses (party_id, address_type);

-- ── party_contacts ─────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_contacts_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_contacts_company_id_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_contacts_party_id
    ON party.party_contacts (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_contacts_party_id_is_primary
    ON party.party_contacts (party_id, is_primary);

-- ── party_emails ───────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_emails_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_emails_company_id_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_emails_party_id
    ON party.party_emails (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_emails_party_id_is_primary
    ON party.party_emails (party_id, is_primary);

-- ── party_phones ───────────────────────────────────────────────────────────────
DROP INDEX IF EXISTS party.idx_party_phones_company_id_party_id_primary;
DROP INDEX IF EXISTS party.idx_party_phones_company_id_party_id_is_primary;
CREATE UNIQUE INDEX IF NOT EXISTS idx_party_phones_party_id
    ON party.party_phones (party_id) WHERE is_primary AND (metadata ->> 'deleted_at') IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_phones_party_id_is_primary
    ON party.party_phones (party_id, is_primary);

-- Drop supporting indexes + the column.
DROP INDEX IF EXISTS party.idx_parties_company_id;
DROP INDEX IF EXISTS party.idx_party_addresses_company_id;
DROP INDEX IF EXISTS party.idx_party_contacts_company_id;
DROP INDEX IF EXISTS party.idx_party_emails_company_id;
DROP INDEX IF EXISTS party.idx_party_phones_company_id;

ALTER TABLE party.parties         DROP COLUMN IF EXISTS company_id;
ALTER TABLE party.party_addresses DROP COLUMN IF EXISTS company_id;
ALTER TABLE party.party_contacts  DROP COLUMN IF EXISTS company_id;
ALTER TABLE party.party_emails    DROP COLUMN IF EXISTS company_id;
ALTER TABLE party.party_phones    DROP COLUMN IF EXISTS company_id;
