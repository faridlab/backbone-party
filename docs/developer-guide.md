<!-- Reader: App developer · Mode: Tutorial → How-to -->
# Developer Guide

Compose `backbone-party` into your Axum service and register your first party. This is for the
engineer *building on* the module, not maintaining it — you treat the crate as a black box that
hands you a `Router` and a set of services.

For the deep integration contract (public surface, event sync, logical-FK rules) see the
[Extension guide](extension-guide.md); this page gets you running first.

## Install

Add the crate to your service's `Cargo.toml`. In the metaphor workspace, reference it by path; if you
consume it from its own git repository, use a `git`/`tag` source instead. (The module itself pulls
the Backbone framework crates from git — see its [`Cargo.toml`](../Cargo.toml) — so it builds anywhere
without path fix-up.)

```toml
[dependencies]
# In the metaphor workspace (sibling projects):
backbone-party = { path = "../backbone-party" }
# Or from git, pinned for reproducible builds:
# backbone-party = { git = "<your-backbone-party-repo>", tag = "v0.1.2" }
```

You also need a running PostgreSQL and its migrations applied (the module owns the `party` schema):

```bash
DATABASE_URL="postgres://user:pass@localhost/erp" metaphor migration run
```

## Quickstart — mount it and create a party

**1. Wire the module into your router** (`main.rs` of your `backend-service`):

```rust
use backbone_party::{PartyModule, create_guarded_party_routes};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    let party = PartyModule::builder()
        .with_database(pool.clone())
        .build()?;

    // RECOMMENDED: read + validated create only. Do NOT use all_crud_routes() in production.
    let app = axum::Router::new().merge(create_guarded_party_routes(&party));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

**2. Create a party** (organization, with a tax ID):

```bash
curl -sS -X POST http://localhost:8080/parties \
  -H 'Content-Type: application/json' \
  -d '{
        "partyCode": "ACME-001",
        "partyKind": "organization",
        "name": "Acme Nusantara",
        "legalName": "PT Acme Nusantara",
        "npwp": "012345678901234"
      }'
```

Expected — `201 Created`:

```json
{ "id": "3f2504e0-4f89-41d3-9a0c-0305e82c3301" }
```

**3. Add its primary address** (geo ids are optional and stored opaquely):

```bash
curl -sS -X POST http://localhost:8080/party-addresses \
  -H 'Content-Type: application/json' \
  -d '{ "partyId": "3f2504e0-4f89-41d3-9a0c-0305e82c3301",
        "addressType": "office", "line1": "Jl. Sudirman 1",
        "postalCode": "10210", "isPrimary": true, "isBilling": true }'
```

> Request bodies are **camelCase** (`partyCode`, `partyId`, `isPrimary`) — the guarded handlers use
> `#[serde(rename_all = "camelCase")]`.

## Key concepts

Five ideas before you go further. One line each; the "why" links to [Philosophy](philosophy.md).

- **Party = identity, not a role.** You never set `is_customer`; you create *your own*
  `Customer` row that references `party_id`. See [Philosophy → roles are projections](philosophy.md#roles-are-projections--the-picture).
- **Logical FK.** Reference `party.Party.id` from your projection; **never** add a DB foreign key
  across the boundary. Same for the geo ids on an address.
- **Guarded vs full surface.** `create_guarded_party_routes` = read + *validated* create. The full
  12-endpoint CRUD (`all_crud_routes()`) skips validation — trusted/admin only.
- **One primary per party per kind.** At most one primary address / email / phone / contact.
  Switch it with `POST /party-set-primary`, don't try to set two.
- **Multi-channel.** A party has *many* addresses, emails, phones, and contacts — each its own row.

## Recipes

### How do I switch which address is primary?

You can't set a second `isPrimary: true` directly (the DB rejects it, `422 duplicate_primary`).
Switch atomically:

```bash
curl -sS -X POST http://localhost:8080/party-set-primary \
  -H 'Content-Type: application/json' \
  -d '{ "partyId": "<party>", "kind": "address", "childId": "<new-primary-address>" }'
```

`kind` is one of `address` | `contact` | `email` | `phone`. It clears the old primary and sets the
new one in one transaction.

### How do I register a person (not an organization)?

A person requires a name part (`firstName` or `lastName`); it may carry a `nik` but not be forced to.
An organization requires `legalName` and must **not** carry a `nik`.

```bash
curl -sS -X POST http://localhost:8080/parties \
  -H 'Content-Type: application/json' \
  -d '{ "partyCode": "P-100", "partyKind": "person",
        "name": "Budi Santoso", "firstName": "Budi", "lastName": "Santoso",
        "nik": "3201234567890123" }'
```

### How do I keep my projection in sync?

Subscribe to `PartyCreated` / `PartyUpdated` / `PartyArchived` and update your `Customer`/`Supplier`
row (eventually consistent). Denormalizing `name`/`partyCode` for display is fine — Party stays the
source of truth. Full contract: [Extension guide](extension-guide.md#consuming-the-multi-master-seam).
> Note: per [ADR-002](adr/ADR-002-data-integrity-invariants.md), these events are **declared but not
> yet published** — until the messaging seam lands, reconcile on read.

### How do I read parties?

The read routes are the generated CRUD GETs, e.g. `GET /parties`, `GET /parties/:id`,
`GET /parties?page=…`. See the route table in
[`party_handler.rs`](../src/presentation/http/party_handler.rs).

## Configuration

Runtime config lives in the **host service**, not the module — the module only needs a `PgPool`.

| Option | Default | When to change |
|--------|---------|----------------|
| `DATABASE_URL` | — (required) | Point at your PostgreSQL; the module reads/writes the `party` schema |
| `with_database(pool)` | — (required) | The only builder input; panics-free `build()` errors if unset |
| Feature flags (`auth`, `events`, `grpc`, `openapi`) | all off (`default = []`) | Turn on `auth` to use `create_protected_party_routes`; others gate optional layers |

Host-level config templates ship under [`config/`](../config/) (`application.yml`,
`application-dev.yml`, `application-prod.yml`) as a reference for db/server/log settings.

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `build()` → `Database pool not configured` | `.with_database(pool)` not called | Pass the `PgPool` before `.build()` |
| `422 inconsistent_party_kind` | person with no name part, or org with a NIK / no legalName | Provide `firstName`/`lastName` (person) or `legalName` and drop `nik` (org) |
| `422 invalid_npwp` / `invalid_nik` | wrong digit count (NPWP 15/16, NIK 16) | Send the correct number of digits; separators are ignored |
| `422 duplicate_primary` | a primary of that kind already exists | Use `POST /party-set-primary` to switch, don't set a second |
| `422 duplicate_party_code` / `duplicate_npwp` / `duplicate_nik` | value already used by a live party | These are unique per non-deleted party; pick another |
| `422 party_not_found` on add-address/email/… | `partyId` doesn't exist (or is soft-deleted) | Create the party first; check the id |
| Writes succeed but skip validation | You mounted `all_crud_routes()` / `routes()` | Switch to `create_guarded_party_routes(&party)` |
| relation `party.parties` does not exist | migrations not applied | `metaphor migration run` with `DATABASE_URL` set |

Error codes are typed in [`party_write_service.rs`](../src/application/service/party_write_service.rs)
(`PartyWriteError::code()`); the numeric behavior is pinned by
[`tests/party_golden_cases.rs`](../tests/party_golden_cases.rs).
