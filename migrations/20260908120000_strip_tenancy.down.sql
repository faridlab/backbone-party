-- Hand-authored (user-owned). Not regenerated.
--
-- Best-effort restore sketch for the tenancy strip (ADR-0029). This is a breaking module
-- release against dev-stage databases: the down re-adds the company_id column as nullable
-- with its plain index and the company isolation policy shape, but restores NO data —
-- rows written after the strip (or after the decorator re-keyed them) carry org_unit_id
-- only. The composing service's tenancy decorator remains the live fence; treat this
-- down as a schema-shape sketch for archaeology, not a usable rollback.

ALTER TABLE party.parties        ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE party.party_addresses ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE party.party_contacts  ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE party.party_emails    ADD COLUMN IF NOT EXISTS company_id uuid;
ALTER TABLE party.party_phones    ADD COLUMN IF NOT EXISTS company_id uuid;

-- The strip's restored domain one-primary uniques go away again (the company-leading
-- variants would need company data this sketch does not restore).
DROP INDEX IF EXISTS party.idx_party_addresses_party_id;
DROP INDEX IF EXISTS party.idx_party_contacts_party_id;
DROP INDEX IF EXISTS party.idx_party_emails_party_id;
DROP INDEX IF EXISTS party.idx_party_phones_party_id;

CREATE INDEX IF NOT EXISTS idx_parties_company_id         ON party.parties (company_id);
CREATE INDEX IF NOT EXISTS idx_party_addresses_company_id ON party.party_addresses (company_id);
CREATE INDEX IF NOT EXISTS idx_party_contacts_company_id  ON party.party_contacts (company_id);
CREATE INDEX IF NOT EXISTS idx_party_emails_company_id    ON party.party_emails (company_id);
CREATE INDEX IF NOT EXISTS idx_party_phones_company_id    ON party.party_phones (company_id);
