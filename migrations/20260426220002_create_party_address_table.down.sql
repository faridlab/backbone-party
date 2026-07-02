-- Down: drop party.party_addresses table
DROP TABLE IF EXISTS party.party_addresses CASCADE;
DROP FUNCTION IF EXISTS party.party_addresses_audit_timestamp() CASCADE;
