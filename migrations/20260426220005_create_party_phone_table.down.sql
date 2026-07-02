-- Down: drop party.party_phones table
DROP TABLE IF EXISTS party.party_phones CASCADE;
DROP FUNCTION IF EXISTS party.party_phones_audit_timestamp() CASCADE;
