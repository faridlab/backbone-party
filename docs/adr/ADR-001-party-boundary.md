# ADR-001: The party bounded context (identity, roles-as-projections)

**Status**: Accepted — **Applied 2026-07-02**
**Deciders**: Farid (owner)
**Related**: `docs/erp/shared-masters-ownership.md`, `docs/erp/relationship-crm.md`; adopted the
multi-channel model from VINSTEKNIK's `salt-laravel-contacts`.

## Context

Accounting AR/AP, selling, buying, and CRM all need one canonical party identity. ERPNext models
`Customer` and `Supplier` as near-duplicate party records cross-linked by hacks; lifting them whole
imports that overloading. `backbone-accounting` already emits `party_id` as a logical FK to a
`party.Party` that did not yet exist. The workspace ownership ledger assigns the Party master to
`backbone-party` with core `party_code, legal_name, party_kind, NPWP/NIK, status`, and consuming
contexts holding projections.

## Decision

1. **`backbone-party` owns the canonical identity only** — Party (person or organization) + its
   multi-channel address book (Address, Contact, Email, Phone). Core fields per the ownership
   ledger: `party_code`, `party_kind`, `name`/`legal_name`, `npwp`/`nik`, `status`.
2. **Roles are projections, not flags.** There is no `is_customer`/`is_supplier` on Party. Being a
   customer = having a `selling.Customer` projection; supplier = `buying.Supplier`; employee =
   payroll. Each references `party_id` and holds only the fields it needs.
3. **A Lead/Prospect is not a Party.** CRM owns them provisionally; conversion is an explicit ACL
   step that *creates* a canonical Party and links back.
4. **Addresses are structured against `backbone-geo` by logical FK.** `PartyAddress` carries
   country/province/city/district/subdistrict ids as `@exclude_from_foreign_key_check` refs — no DB
   constraint across schemas, validated at the ACL layer / consuming service, never joined to geo
   from here. Adopted the multi-channel decomposition (N addresses/emails/phones/contacts) from
   `salt-laravel-contacts`, reframed from a personal address book to an ERP party.
5. **Indonesia-first statutory identity.** `npwp` (15/16 digits) and `nik` (16 digits) are
   validated, unique, first-class.
6. **Dedicated `party` Postgres schema**; cross-module references are logical FKs + events
   (`PartyCreated/Updated/Archived`) for eventual-consistency projection sync.

## Consequences

- Fulfills accounting's dangling `party_id` seam; AR/AP aging resolves a real party.
- One identity backs Customer, Supplier, Employee, and CRM views without a god-entity.
- Addresses become structured `subdistrict_id` references into the shared geo hierarchy rather than
  free text.
- Parking lot: socials/URLs channels, party groups/tags, per-company scoping of a party, credit
  terms (a selling/accounting concern), and the CRM Lead→Party conversion ACL (owned by CRM).
