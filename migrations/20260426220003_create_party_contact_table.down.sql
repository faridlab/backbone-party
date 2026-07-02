-- Down: drop party.party_contacts table
DROP TABLE IF EXISTS party.party_contacts CASCADE;
DROP FUNCTION IF EXISTS party.party_contacts_audit_timestamp() CASCADE;
