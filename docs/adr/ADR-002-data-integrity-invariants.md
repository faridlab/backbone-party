# ADR-002: Party data-integrity invariants (one-primary + kind coherence)

**Status**: Accepted — **Applied 2026-07-02**
**Deciders**: Farid (owner), council (module:backbone-party, focus=maturity, 2026-07-02)
**Related**: ADR-001 (party boundary)

## Context

A maturity council confirmed the CRUD-bypass security story is sealed (guarded routes lock generic
writes; the `all_crud_routes()`/`#[deprecated] routes()` seal is inherited). But it found a HIGH
**data-integrity** gap: the guarded write path validated *syntax* (NPWP/NIK digit count) and
*parentage* (party exists) but never *coherence* or *cardinality*. Three invariants a Tier-0 master
must hold had no enforcer at any layer, and the test suite **codified the broken states as passing**:

1. **One primary per party** — every `(party_id, is_primary)` index was plain/non-unique, so two
   `is_primary=true` addresses (or emails/phones/contacts) was a legal state. Billing/shipping
   resolution then becomes non-deterministic (`LIMIT 1` on an unordered set), with no error. And
   because the guarded surface is create-only, a duplicate primary could never be demoted.
2. **Kind/field coherence** — `create_party` made `name` the only required identity field and
   defaulted `party_kind` to `organization`; a `person` with no name parts, or an `organization`
   carrying a NIK and no `legal_name`, was accepted `status=active`. Ambiguous legal identity then
   flows into the tax/ledger seam permanently.
3. **Geo linkage** — geo ids are stored opaque (logical FK by design); the promised ACL validator
   does not exist in any consumer yet.

## Decision

1. **One-primary-per-party is a DB invariant.** A partial-unique index
   `UNIQUE (party_id) WHERE is_primary AND deleted_at IS NULL` on all four children
   (PartyAddress/Contact/Email/Phone). `add_*` maps the violation to a typed `DuplicatePrimary`
   (`422 duplicate_primary`).
2. **Primary is switchable.** `set_primary(party, kind, child_id)` (`POST /party-set-primary`)
   clears `is_primary` on the party's children of that kind then sets it on the target, in one
   transaction (clear-then-set, so the index never sees two primaries mid-tx). This resolves the
   "create-only ⇒ can't demote" corner the invariant would otherwise create.
3. **Kind/field coherence at create.** `create_party` branches on `party_kind`: a **person**
   requires `first_name` or `last_name`; an **organization** requires `legal_name` and must not
   carry a `nik`. Violations → `422 inconsistent_party_kind`.
4. **Geo validation is explicitly a consumer/ACL concern** (parked). Party keeps geo ids opaque;
   the tests document this as the logical-FK boundary, not a validation gap party owns.

## Consequences

- Deterministic billing/shipping/contact resolution: exactly one primary per party per kind, and it
  can be switched.
- No ambiguous person/organization identities enter the master.
- Three new route-level probes (IGC-5/6/7) lock the invariants; the suite no longer certifies broken
  states.
- Residual / parking lot (per the council): NPWP/NIK **checksum** (digit-count is today's ceiling);
  **projection-event emission** (`PartyCreated/Updated/Archived` are declared, not yet published —
  a messaging-seam task); **roles-referential guarantee** (AR/AP posting against a party that is
  neither a Customer nor Supplier projection — belongs to the consuming/accounting council); a full
  **validated update/archive** path; **per-company scoping** (needs a product trigger).
