# backbone-party — PRD

> Tier-0 master-data module. Owns the **canonical party identity** every module references — a
> person or an organization — plus its multi-channel address book. Indonesia-first.

## Problem

Accounting AR/AP, selling, buying, and CRM all need one stable answer to "who is this party?" In
ERPNext, `Customer` and `Supplier` are near-duplicate party records cross-linked by hacks; lifting
them whole imports that overloading. We need one canonical Party identity, with the customer /
supplier / employee facets expressed as **projections** in the consuming contexts — not flags here.
`backbone-accounting` already emits `party_id` as a logical FK to a `party.Party` that must exist.

## Scope

**In:**
- `Party` — canonical identity: `party_code`, `party_kind` (person | organization), `name`,
  `legal_name`, person name parts, **NPWP** (tax id) + **NIK** (national ID), `status`.
- Multi-channel address book (adopted from `salt-laravel-contacts`): `PartyAddress` (structured
  against `backbone-geo`), `PartyContact` (contact persons), `PartyEmail`, `PartyPhone`.
- Validated write path + guarded routes.

**Out (projections / other modules):**
- **Customer view** → `selling`/`crm`; **Supplier view** → `buying`; **Employee** → `payroll`.
  These reference `party_id` and hold only the fields they need.
- **Lead / Prospect** → `backbone-crm` (a Lead is *not* a Party; conversion is an ACL step that
  mints a Party).
- **Pricing / credit terms / loyalty** → the consuming context.
- **Geo hierarchy** → `backbone-geo` (referenced by logical FK on addresses).

## Personas
- **Master-data admin** — registers parties + address book.
- **Consuming modules** — reference `party.Party.id`, create/update their own projection on
  `PartyCreated/Updated/Archived` events.

## Success criteria
- One canonical party identity; roles are projections, never re-owned.
- NPWP/NIK validated + unique; party_code unique. Addresses structured against geo by logical FK.
- Dedicated `party` Postgres schema; zero horizontal Cargo edges; referenced by logical FK only.
- Fulfills accounting's `party_id` seam.

## Indonesia-first notes
`npwp` (org/individual tax id, 15/16 digits) + `nik` (person national ID, 16 digits) are first-class,
validated, unique. Addresses resolve to the Indonesian *wilayah* via `backbone-geo` ids.
