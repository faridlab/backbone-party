# Business Flow — Party & Address Book

> Owning module: `backbone-party` · Implemented in
> `src/application/service/party_write_service.rs`, enforced by
> `src/presentation/http/guarded_routes.rs`, proven by `tests/party_golden_cases.rs`.
> Rules: R1–R8 in `schema/hooks/party.hook.yaml`. Ownership: `docs/erp/shared-masters-ownership.md`.

Party is the **canonical identity** of a person or organization the whole ERP references. It is NOT
a customer or a supplier — those are **projections** owned by the consuming context
(`selling.Customer`, `buying.Supplier`, payroll). A CRM Lead is not a Party; conversion is an ACL
step that mints one.

## Actors
- **Master-data admin** — registers parties + their address book.
- **Consuming modules** (accounting AR/AP, selling, buying, crm) — reference `party.Party.id` as a
  logical FK and hold their own projection.

## Flows

### Register a party
- `POST /parties` `{ partyCode, partyKind (person|organization), name, legalName?, firstName?,
  lastName?, npwp?, nik? }`.
- Rules: R1 NPWP format (15/16 digits) → `invalid_npwp`; R2 NIK format (16 digits) → `invalid_nik`;
  R3 unique party_code → `duplicate_party_code`; R4 unique NPWP → `duplicate_npwp`; R5 unique NIK →
  `duplicate_nik`. → `201 { id }`.

### Add an address (structured against geo)
- `POST /party-addresses` `{ partyId, addressType, line1, line2?, countryId?, provinceId?, cityId?,
  districtId?, subdistrictId?, postalCode?, latitude?, longitude?, isPrimary?, isBilling?,
  isShipping? }`.
- Rule R6: party must exist → `party_not_found`. The geo ids are **logical FKs** to `backbone-geo`
  (`geo.Country/Province/City/District/Subdistrict.id`) — stored verbatim, validated at the ACL
  layer / consuming service, never joined to geo's schema from here (R8). → `201 { id }`.

### Add a contact person / email / phone
- `POST /party-contacts` (name, jobTitle?, department?, email?, phone?), `POST /party-emails`
  (label, email, isPrimary), `POST /party-phones` (label, phone, isPrimary). Each requires an
  existing party (R6); email must contain `@` (R7). Multi-value per party.

## The accounting seam (now fulfilled)
`backbone-accounting`'s `Ledger`/`Journal` lines carry `party_id` documented as *"logical FK to
party.Party.id (customer/supplier)"* for AR/AP aging. Party now exists as that anchor; accounting
resolves a party by id without importing this module.

## Not here (projections / other modules)
Customer/Supplier/Employee views (selling/buying/payroll), Lead/Prospect (crm), pricing, loyalty,
credit terms. See exact numbers in [golden-cases.md](golden-cases.md).
