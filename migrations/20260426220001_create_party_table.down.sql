-- Down: drop party.parties table
DROP TABLE IF EXISTS party.parties CASCADE;
DROP FUNCTION IF EXISTS party.parties_audit_timestamp() CASCADE;
