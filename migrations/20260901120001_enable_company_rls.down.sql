-- Down: remove the company RLS fence for party module

-- Reverse the company RLS fence for party.parties
DROP POLICY IF EXISTS parties_company_isolation ON party.parties;
ALTER TABLE party.parties NO FORCE ROW LEVEL SECURITY;
ALTER TABLE party.parties DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for party.party_addresses
DROP POLICY IF EXISTS party_addresses_company_isolation ON party.party_addresses;
ALTER TABLE party.party_addresses NO FORCE ROW LEVEL SECURITY;
ALTER TABLE party.party_addresses DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for party.party_contacts
DROP POLICY IF EXISTS party_contacts_company_isolation ON party.party_contacts;
ALTER TABLE party.party_contacts NO FORCE ROW LEVEL SECURITY;
ALTER TABLE party.party_contacts DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for party.party_emails
DROP POLICY IF EXISTS party_emails_company_isolation ON party.party_emails;
ALTER TABLE party.party_emails NO FORCE ROW LEVEL SECURITY;
ALTER TABLE party.party_emails DISABLE ROW LEVEL SECURITY;

-- Reverse the company RLS fence for party.party_phones
DROP POLICY IF EXISTS party_phones_company_isolation ON party.party_phones;
ALTER TABLE party.party_phones NO FORCE ROW LEVEL SECURITY;
ALTER TABLE party.party_phones DISABLE ROW LEVEL SECURITY;

