-- Down: drop the parties VAT column (values were validated-only, no dependent objects).
ALTER TABLE party.parties DROP COLUMN IF EXISTS vat;
