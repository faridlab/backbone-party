<!-- Reader: All · Mode: Navigation -->
# backbone-party — Handbook

The canonical **party identity** for the Backbone ERP: one person-or-organization record every
other module references by `party_id`, plus its multi-channel address book. Customer / Supplier /
Employee are *not* here — they are projections owned by the consuming context. Indonesia-first
(NPWP + NIK are first-class). Schema YAML is the single source of truth; most of the crate is
generated from it.

This page is the map. Every doc below targets **one reader**. Find your row, start there.

## Start here by who you are

| You are… | You want… | Read, in order |
|----------|-----------|----------------|
| **Evaluating** the module | Why it exists, what it refuses to do | [Philosophy](philosophy.md) → [PRD](prd.md) → [ADR-001](adr/ADR-001-party-boundary.md) |
| **Building an app** on it | Install → first party in 15 min → recipes | [Developer guide](developer-guide.md) → [Extension guide](extension-guide.md) |
| **Maintaining / extending** it | How the machine works, how to add a field safely | [Architecture](architecture.md) → [Maintainer guide](maintainer-guide.md) |
| **Anyone** | One term, one meaning | [Glossary](glossary.md) |

## The handbook (canonical sections)

1. **[Philosophy & motivation](philosophy.md)** — *Evaluator.* Identity-vs-projections, schema-as-SSoT,
   Indonesia-first, decoupling by logical FK. What the module deliberately does **not** do.
2. **Background & prior art** — *Evaluator.* Folded into [Philosophy → "What we borrowed"](philosophy.md#what-we-borrowed-and-what-we-rejected)
   (ERPNext's overloaded Customer/Supplier; `salt-laravel-contacts`' multi-channel decomposition).
3. **Technology & the "why"** — *Evaluator + Maintainer.* [Architecture → "The stack, and why"](architecture.md#5-the-stack-and-why).
4. **[Architecture](architecture.md)** — *Maintainer.* C4 context/containers, the DDD 4-layer shape,
   a create-party request traced end-to-end. Mermaid diagrams.
5. **[Maintainer guide](maintainer-guide.md)** — *Maintainer.* Schema SSoT + regeneration, the
   `// <<< CUSTOM` markers, where code goes per layer, and two worked walkthroughs
   (add a field; add a validated write rule).
6. **[Developer guide](developer-guide.md)** — *App developer.* Install → quickstart → key concepts →
   recipes → configuration → troubleshooting. Every command was run.
7. **Contribution guide** — *Contributor.* **Not yet written** (stub). Until it lands: conventional
   commits, **no Claude / Co-authored-by signatures** (see root `CLAUDE.md`); run
   `metaphor schema schema validate` and `metaphor dev test` before a PR.
8. **[Glossary](glossary.md)** — *All.* The ubiquitous language (Party, projection, logical FK,
   primary, NPWP/NIK, guarded routes…). Everything else defers to it.
9. **ADRs** — *Maintainer.* [ADR-001 — party boundary](adr/ADR-001-party-boundary.md) ·
   [ADR-002 — data-integrity invariants](adr/ADR-002-data-integrity-invariants.md). Immutable once
   accepted; supersede, don't edit.

## Reference material (already in-repo)

These are *reference mode* — exhaustive and dry. Consult them, don't read them front-to-back.

- **[FSD](fsd.md)** — entity/table/endpoint tables, the R1–R8 rule list, integration seams.
- **[Business flows](business-flows/party.md)** + **[golden cases](business-flows/golden-cases.md)** —
  each feature as actors → flow → rules → postconditions, with the numeric oracle.
- **[Schema DSL reference](schema/README.md)** — the model/hook/workflow YAML grammar, types,
  generators, error codes. This is framework-wide, not party-specific.
- **Schema SSoT** — [`schema/models/`](../schema/models/) (`*.model.yaml`) and
  [`schema/hooks/party.hook.yaml`](../schema/hooks/party.hook.yaml).

## The one rule that governs everything

> **Edit the schema YAML, never the generated `.rs`** — unless you are inside a
> `// <<< CUSTOM … // END CUSTOM` marker or in a file listed `user_owned` in
> [`metaphor.codegen.yaml`](../metaphor.codegen.yaml). Everything else is overwritten on the next
> `metaphor schema schema generate`. The [Maintainer guide](maintainer-guide.md) explains the seam.
