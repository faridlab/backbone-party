<!-- Reader: Maintainer · Mode: How-to -->
# Maintainer Guide

How to change `backbone-party` without breaking the regeneration contract. If you edit a generated
`.rs` file outside a `// <<< CUSTOM` marker, your change is silently overwritten on the next
`metaphor schema schema generate` — this guide is how to avoid that.

Assumes you have read [Architecture](architecture.md) (the 4-layer shape) and the project
[`CLAUDE.md`](../CLAUDE.md). This is a `module` project: **schema YAML is the source of truth.**

## The regeneration contract

Three kinds of code live in `src/`. Know which one you're touching *before* you type.

| Kind | Where | Survives regen? | You edit it… |
|------|-------|-----------------|--------------|
| **Generated** | Most of `src/` — entities, DTOs, `*_service.rs` aliases, `*_repository.rs`, `*_handler.rs`, `routes/generated.rs` | ❌ overwritten | never directly — change the **schema YAML** |
| **CUSTOM markers** | Inside `// <<< CUSTOM … // END CUSTOM` blocks in generated files (`lib.rs`, `presentation/http/mod.rs`, `application/service/mod.rs`) | ✅ preserved | for re-exports/declarations that must sit next to generated code |
| **`user_owned`** | Whole files globbed in [`metaphor.codegen.yaml`](../metaphor.codegen.yaml) | ✅ skipped wholesale | freely — the generator never reads them |

Today the `user_owned` set is: `party_write_service.rs`, `guarded_routes.rs`, the tests
(`party_golden_cases.rs`, `integrity_probes.rs`, `tests/features/**`), and `docs/**`.

> **Marker syntax is `// <<< CUSTOM` … `// END CUSTOM`.** (The generic skill template shows
> `// CUSTOM >>>` — this codebase uses `// END CUSTOM`; match what's in the files.)

## Where new code goes

| I need to add… | Put it… | Regen-safe by… |
|----------------|---------|----------------|
| A field / index / enum variant | `schema/models/<entity>.model.yaml` | it *is* the SSoT |
| A new entity | new `schema/models/<name>.model.yaml` + add to `imports:` in `index.model.yaml` | SSoT |
| A validated write rule | `application/service/party_write_service.rs` | `user_owned` |
| A non-CRUD / guarded endpoint | `presentation/http/guarded_routes.rs` | `user_owned` |
| A re-export of the above | inside `// <<< CUSTOM` in `lib.rs` / `http/mod.rs` | marker |
| A behavior test | `tests/party_golden_cases.rs` or `tests/integrity_probes.rs` | `user_owned` |

**Never** add a `main.rs` (this is a library), hand-roll an Axum CRUD route (use `BackboneCrudHandler`),
or add a DB foreign key across the module boundary (geo/consumer refs are logical FKs).

## Before you touch anything

```bash
metaphor schema schema validate     # schema parses & is consistent  → "All schemas are valid"
metaphor schema schema doctor       # hand-written aggregators match the schema → "no drift detected"
```

Both are verified to run clean in this module today.

## Walkthrough A — add a field to Party (regen-safe)

Goal: add an optional `website` field to `Party`.

1. **Edit the schema** — [`schema/models/party.model.yaml`](../schema/models/party.model.yaml), in
   `fields:`:

   ```yaml
   website:
     type: string?
     attributes: ["@max(255)"]
     description: "Public website URL (organization)"
   ```

2. **Validate:**

   ```bash
   metaphor schema schema validate
   ```

3. **Generate a migration for the change**, then regenerate the Rust:

   ```bash
   metaphor schema schema migration      # emits an ALTER migration under migrations/
   metaphor schema schema generate       # regenerates entity, DTOs, repo, service, handler
   ```

   (Equivalently, `metaphor migration alter` generates the ALTER TABLE from the schema diff — see
   `metaphor migration --help`.)

4. **The generated entity/DTO/handler now carry `website`.** You wrote no Rust. If you need
   `website` *validated* on create, add it to `NewParty` + `create_party` in
   `party_write_service.rs` (that file is `user_owned`) and to `CreatePartyBody` in
   `guarded_routes.rs`.

5. **Migrate & test:**

   ```bash
   metaphor migration run        # apply pending migrations (needs DATABASE_URL)
   metaphor dev test             # unit + integration + behavior
   ```

## Walkthrough B — add a validated write rule

Goal: reject a party whose `party_code` is shorter than 3 characters (`422 invalid_party_code`).
This is *domain* logic, so it lives in the hand-written write path — not the schema.

1. **Add the error variant** in `party_write_service.rs` (`PartyWriteError` enum + its `code()` arm;
   `http_status()` already maps every non-`Db` variant to `422`):

   ```rust
   InvalidPartyCode(String),
   // …in code(): PartyWriteError::InvalidPartyCode(_) => "invalid_party_code",
   ```

2. **Enforce it** at the top of `create_party`, before the INSERT — mirror the existing
   `validate_npwp` / kind-coherence guards:

   ```rust
   if p.party_code.trim().len() < 3 {
       return Err(PartyWriteError::InvalidPartyCode(p.party_code));
   }
   ```

3. **Prove it** — add a case to [`tests/party_golden_cases.rs`](../tests/party_golden_cases.rs) (the
   oracle) asserting the `422 invalid_party_code`. Both the service and its test are `user_owned`, so
   they survive regeneration.

4. `metaphor dev test`.

Because `create_party` is the *only* mounted write path for parties (the guarded router does not
mount generic create), this rule cannot be bypassed by a client. That is the whole point of the
guarded surface — see [Architecture §2](architecture.md#2-containers).

## Migrations

Migrations are timestamped `NNNNNNNNNNNNNN_description.up.sql` / `.down.sql` under `migrations/`, and
every table is qualified into the `party` schema (`CREATE SCHEMA party`, `party.parties`, …).

```bash
metaphor migration list        # what exists
metaphor migration status      # applied vs pending, across modules
metaphor migration run         # apply pending (DATABASE_URL required)
```

Prefer generating migrations from the schema (`metaphor schema schema migration` /
`metaphor migration alter`) over hand-writing DDL, so the SQL and the schema YAML never drift. When
you *must* hand-write (e.g. a partial-unique index like the one-primary invariant), keep the `.down`
exact and reversible. See the [database-migration-specialist skill] for the safety rules.

[database-migration-specialist skill]: ../CLAUDE.md

## Versioning & release

- The crate is `backbone-party` `0.1.x` ([`Cargo.toml`](../Cargo.toml)); bump per conventional
  commits.
- Green gate before release: `metaphor schema schema validate && metaphor dev test`.
- Commits: **conventional commits, no Claude / Co-authored-by signatures** (root `CLAUDE.md`).
- The public surface consumers depend on (entities, DTOs, `PartyWriteService`,
  `create_guarded_party_routes`, `validate_npwp`/`validate_nik`) is documented in the
  [Extension guide](extension-guide.md) — treat changes to it as breaking.

## What will break things

- Editing generated code **outside** a `// <<< CUSTOM` marker → gone on next generate.
- Adding a field to the entity `.rs` instead of the schema YAML → the migration and DTO won't match.
- A DB foreign key from an address's `country_id` to `geo.countries` → couples the schemas; geo ids
  are **logical** FKs (`@exclude_from_foreign_key_check`).
- Mounting `PartyModule::routes()` / `all_crud_routes()` in production → exposes unvalidated writes.
- Editing `src/module.rs` → it's dead skeleton code (see [Architecture §3](architecture.md#3-components--the-ddd-4-layer-shape)); nothing reads it.
