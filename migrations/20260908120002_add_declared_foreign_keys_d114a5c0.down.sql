-- Down: drop the declared foreign keys for party module
ALTER TABLE IF EXISTS party.party_phones DROP CONSTRAINT IF EXISTS fk_party_phones_party_id;
ALTER TABLE IF EXISTS party.party_emails DROP CONSTRAINT IF EXISTS fk_party_emails_party_id;
ALTER TABLE IF EXISTS party.party_contacts DROP CONSTRAINT IF EXISTS fk_party_contacts_party_id;
ALTER TABLE IF EXISTS party.party_addresses DROP CONSTRAINT IF EXISTS fk_party_addresses_party_id;
