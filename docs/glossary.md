<!-- Reader: All · Mode: Reference -->
# Glossary — Ubiquitous Language

One term, one meaning, used consistently across every doc, the schema YAML, and the code. If a term
here disagrees with a doc, this page wins — fix the doc. New shared term? Add it here first.

### Party
The canonical identity of **a person or an organization** that the ERP transacts with. Table
`party.parties`. Identified externally by `party_code` (unique) and internally by `id` (UUID). It is
**not** a customer, supplier, or employee — those are *projections*. Owner: this module.

### Projection
A context-owned record that references a `party_id` and holds only the fields that context needs —
`selling.Customer`, `buying.Supplier`, payroll `Employee`. "Being a customer" *is* having a
`Customer` projection. Party never knows its projections exist. See [Philosophy](philosophy.md#roles-are-projections--the-picture).

### Party kind
The `PartyKind` enum: `organization` (default) or `person`. Drives coherence rules — a person needs a
name part; an organization needs a `legal_name` and must not carry a `nik`.

### Logical FK
A cross-module / cross-schema reference stored as a plain id with **no database foreign-key
constraint** and **no Cargo dependency**. Validated (if at all) at the consumer's ACL layer, never
by a DB join. In the schema, marked `@exclude_from_foreign_key_check`. Applies to a consumer's
`party_id`, and to an address's `country_id`/`province_id`/`city_id`/`district_id`/`subdistrict_id`
(→ `backbone-geo`) and audit `created_by`/`updated_by`/`deleted_by` (→ `backbone-sapiens`).

### Address book (multi-channel)
A Party's collection of **PartyAddress**, **PartyContact**, **PartyEmail**, **PartyPhone** — each a
separate table, many per party. Adopted (decomposition only) from `salt-laravel-contacts`.

### Primary
The single designated default child of a given kind for a party — e.g. one primary address, one
primary email. Enforced by a **partial-unique index** `UNIQUE (party_id) WHERE is_primary AND
deleted_at IS NULL` on each child table. Switch it (never set two) via `POST /party-set-primary`
([ADR-002](adr/ADR-002-data-integrity-invariants.md)).

### NPWP
*Nomor Pokok Wajib Pajak* — Indonesian tax ID. 15 (legacy) or 16 (NIK-based) digits, separators
ignored. On Party: optional, unique when present, format-validated (`validate_npwp`).

### NIK
*Nomor Induk Kependudukan* — Indonesian national ID for a **person**, exactly 16 digits. On Party:
optional, unique when present, format-validated (`validate_nik`). An organization may **not** carry
one.

### Wilayah / geo hierarchy
The Indonesian administrative hierarchy — country → province → city → district → subdistrict — owned
by `backbone-geo`. An address references it by logical FK ids; Party stores them opaquely and never
joins to geo.

### Guarded routes
The **recommended** HTTP surface: `create_guarded_party_routes(&PartyModule)` — read routes for every
entity plus **validated** `POST` create via `PartyWriteService`, plus `POST /party-set-primary`.
Generic update/delete/upsert/bulk are not mounted.

### Full / unguarded surface
`PartyModule::all_crud_routes()` — all 12 generated CRUD endpoints per entity with **no domain
validation**. Trusted/admin/seeding only. `routes()` is its `#[deprecated]` alias.

### BackboneCrudHandler
The framework helper that wires the 12 standard CRUD endpoints for an entity from its service + DTOs
(list / create / get / update / patch / soft-delete / restore / empty-trash / bulk-create / upsert /
find-by-id / list-deleted). Generated `*_handler.rs` files call it.

### GenericCrudService / GenericCrudRepository
Framework generics the module specializes by **type alias** (`PartyService = GenericCrudService<…>`)
and thin **newtype** (`PartyRepository(GenericCrudRepository<Party, PgPool>)`). You don't hand-roll
CRUD; you extend these.

### PartyWriteService
The **hand-written** (`user_owned`) validated write path — `create_party`, `add_address`,
`add_contact`, `add_email`, `add_phone`, `set_primary`. Closes the CRUD-bypass by validating format,
uniqueness, kind-coherence, and parentage before insert. Errors are typed as `PartyWriteError`
(each with a `code()` and `http_status()`).

### Schema YAML (SSoT)
`schema/models/*.model.yaml` — the **single source of truth**. Entity, DTOs, migration, repository,
service, handler, and routes are generated from it. Hooks/rules live in
`schema/hooks/party.hook.yaml`. See the [DSL reference](schema/README.md).

### CUSTOM marker
A `// <<< CUSTOM … // END CUSTOM` block inside a generated file whose contents are **preserved**
across regeneration. Used for re-exports/declarations that must sit next to generated code.

### user_owned
Files globbed in [`metaphor.codegen.yaml`](../metaphor.codegen.yaml) that the generator **skips
wholesale** — never reads, merges, or deletes. Currently `party_write_service.rs`,
`guarded_routes.rs`, the behavior tests, and `docs/**`.

### party (Postgres) schema
The dedicated database schema this module owns — `CREATE SCHEMA party`; every table is
`party.<table>`. No other module writes to it; cross-schema references are logical FKs only.

### Soft delete
Deletion recorded as a `deleted_at` timestamp in the `metadata` JSONB rather than a row removal.
Unique constraints and "party exists" checks are `deleted_at IS NULL`-aware.

### R1–R8
The numbered validated-write rules (NPWP/NIK format, unique code/npwp/nik, party existence, email
`@`, geo-as-logical-FK) defined in `schema/hooks/party.hook.yaml` and proven by the
[golden cases](business-flows/golden-cases.md).
