-- Migration: add the cross-border VAT number to parties
-- Hand-authored (user-owned). Not regenerated.
--
-- Adds party.parties.vat — the country-prefixed cross-border VAT number (e.g. DE123456789).
-- Validation is fail-closed at the owner's write path (PartyWriteService::create_party, see
-- src/application/service/party_vat_validation.rs): unknown-country prefixes are refused
-- loudly, '/' is the explicit no-VAT sentinel, and no DB constraint duplicates that logic —
-- the write path is the single validated entry (the generated CRUD surface is not mounted
-- for writes on the guarded router).
--
-- No unique index on vat: two parties in one company may legitimately share a VAT number
-- (a billing contact at the same legal entity); uniqueness is enforced for npwp/nik, which
-- are the Indonesia-first statutory identifiers.

ALTER TABLE party.parties ADD COLUMN IF NOT EXISTS vat TEXT;
