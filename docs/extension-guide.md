# backbone-party — Extension Guide

How a consuming service composes and references this module.

## Composing into a service

```rust
use backbone_party::{PartyModule, create_guarded_party_routes};

let party = PartyModule::builder().with_database(pool.clone()).build()?;

// RECOMMENDED: read + validated create only (no generic patch/delete/upsert/bulk).
let app = axum::Router::new().merge(create_guarded_party_routes(&party));
```

For trusted admin/seed tooling only, `PartyModule::all_crud_routes()` exposes generic CRUD
(`routes()` is a `#[deprecated]` alias) — do not mount it in production.

## Public / stable surface
- **Entities & DTOs** — Party + PartyAddress/Contact/Email/Phone + generated DTOs.
- **Validated write API** — `PartyWriteService`, `NewParty`/`NewAddress`/`NewContact`/`NewEmail`/
  `NewPhone`, `PartyWriteError`, `validate_npwp`/`validate_nik`, `create_guarded_party_routes`.
- **Logical FK identity** — reference `party.Party.id` as `party_id` from your own projection
  (`selling.Customer`, `buying.Supplier`, accounting AR/AP). **Never** add a DB foreign key across
  the module boundary. Roles are projections, not fields on Party.

## Consuming the multi-master seam
- Party addresses reference **backbone-geo** ids (country/province/city/district/subdistrict). Those
  are validated at your ACL layer against geo's read API — party stores them opaquely.
- Subscribe to `PartyCreated/Updated/Archived` to keep your projection in sync (eventually
  consistent). Denormalizing `name`/`party_code` for display is allowed; Party is source of truth.

## Regeneration safety
`src/application/service/party_write_service.rs`, `src/presentation/http/guarded_routes.rs`, tests,
and `docs/**` are `user_owned` in `metaphor.codegen.yaml` and survive
`metaphor schema schema generate --force`. The module owns the `party` Postgres schema.
