-- Down: drop party.party_emails table
DROP TABLE IF EXISTS party.party_emails CASCADE;
DROP FUNCTION IF EXISTS party.party_emails_audit_timestamp() CASCADE;
