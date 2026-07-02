# backbone-party — FSD

Schema (`schema/models/*.model.yaml`) is the SSoT.

## Entities

| Entity | Table | Key | Notes |
|--------|-------|-----|-------|
| Party | `party.parties` | `party_code` unique; `npwp`/`nik` unique when present | `party_kind` (person/organization), name/legal_name, first/last, status. |
| PartyAddress | `party.party_addresses` | — | `party_id`; `address_type`; geo logical FKs (country/province/city/district/subdistrict_id); postal_code, lat/long; is_primary/billing/shipping. |
| PartyContact | `party.party_contacts` | — | `party_id`; contact person (name, job_title, department, email, phone). |
| PartyEmail | `party.party_emails` | — | `party_id`; label, email, is_primary. |
| PartyPhone | `party.party_phones` | — | `party_id`; label, phone, is_primary. |

Tables live in the **`party` Postgres schema**. Soft-delete via `metadata` JSONB.

## Endpoints

- **Guarded (recommended)** — `create_guarded_party_routes(&PartyModule)`: read + **validated
  create** for every entity (`POST /parties`, `/party-addresses`, `/party-contacts`,
  `/party-emails`, `/party-phones`). Generic update/delete/upsert/bulk are not mounted.
- **`PartyModule::all_crud_routes()`** — generated full CRUD (trusted/admin only); `routes()` is the
  `#[deprecated]` alias.

## Validated write rules (R1–R8)
See `schema/hooks/party.hook.yaml`: NPWP (15/16 digits) + NIK (16 digits) format; unique
party_code/npwp/nik; child party existence; email `@`; geo ids are logical FKs (not validated here).

## Integration (logical FKs — no DB FK, no Cargo edge)
- Addresses reference `geo.Country/Province/City/District/Subdistrict.id`.
- Consumers reference `party.Party.id` (`party_id`) — accounting AR/AP, selling.Customer,
  buying.Supplier. Party publishes `PartyCreated/Updated/Archived`; consumers project.

## Behavior specs
- Hooks: `schema/hooks/party.hook.yaml` (Party state machine + R1–R8 + events).
- Workflows: none in-module (Lead→Customer conversion is a CRM saga).
- Flows + oracle: `docs/business-flows/` + `tests/features/party.feature`; executable oracle
  `tests/party_golden_cases.rs` + `tests/integrity_probes.rs`.

## Non-goals
No customer/supplier/employee facets, no leads, no pricing/credit/loyalty, no geo hierarchy.
See [prd.md](prd.md) "Out".
