-- Down: revert the party module to the strict fence (ADR-0014 rollback).
-- Hand-authored (user-owned). Not regenerated.

ALTER TABLE party.parties ENABLE ROW LEVEL SECURITY;
ALTER TABLE party.parties FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS parties_company_isolation ON party.parties;
CREATE POLICY parties_company_isolation ON party.parties
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);

ALTER TABLE party.party_addresses ENABLE ROW LEVEL SECURITY;
ALTER TABLE party.party_addresses FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS party_addresses_company_isolation ON party.party_addresses;
CREATE POLICY party_addresses_company_isolation ON party.party_addresses
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);

ALTER TABLE party.party_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE party.party_contacts FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS party_contacts_company_isolation ON party.party_contacts;
CREATE POLICY party_contacts_company_isolation ON party.party_contacts
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);

ALTER TABLE party.party_emails ENABLE ROW LEVEL SECURITY;
ALTER TABLE party.party_emails FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS party_emails_company_isolation ON party.party_emails;
CREATE POLICY party_emails_company_isolation ON party.party_emails
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);

ALTER TABLE party.party_phones ENABLE ROW LEVEL SECURITY;
ALTER TABLE party.party_phones FORCE  ROW LEVEL SECURITY;
DROP POLICY IF EXISTS party_phones_company_isolation ON party.party_phones;
CREATE POLICY party_phones_company_isolation ON party.party_phones
    FOR ALL
    USING      (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid)
    WITH CHECK (company_id = NULLIF(current_setting('app.company_id', true), '')::uuid);
