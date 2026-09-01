# Fence vocabulary: shared-master unique indexes (the COALESCE recipe)

This module's porting fences use a shared vocabulary. This entry records the
**COALESCE(company_id, nil-uuid) unique-index recipe**: what it is, and the exact places
in this module where it applies.

## The recipe

A per-company partial unique index does not dedupe SHARED rows. Postgres treats `NULL` as
distinct in unique indexes, so with a shared-master posture (ADR-0014 `shared_blank`:
`company_id IS NULL` = one row shared by every company), the plain form lets any number of
shared duplicates through:

```sql
-- WRONG for a shared-capable master: NULL company_id rows never collide.
CREATE UNIQUE INDEX ... ON party.parties (company_id, party_code)
  WHERE (metadata->>'deleted_at') IS NULL;

-- RIGHT: fold the shared bucket (NULL company) into its own sentinel tenant, so there is
-- exactly ONE shared row per business key, alongside one per real company.
CREATE UNIQUE INDEX ... ON party.parties (
    COALESCE(company_id, '00000000-0000-0000-0000-000000000000'::uuid),
    party_code
  )
  WHERE (metadata->>'deleted_at') IS NULL;
```

The sentinel uuid is never a real `organization.companies.id` (all-zero nil), so the
shared bucket cannot collide with a real tenant's. The RLS policy is unaffected — only
the uniqueness domain changes.

## Where it applies in party

Party's master posture is `shared_blank` (the fence migration
`20260816120000_company_fence_shared_blank`): a NULL-company party row is intended to be
the ONE canonical shared identity ("PT Telkom" in the address book), visible to every
company. `company_id` is still `NOT NULL` today — the nullable relaxation is a declared,
deliberately-deferred follow-up (the accepted validator warning in
`schema/models/index.model.yaml` documents this exact state). The recipe applies at that
relaxation, to all three per-company uniques:

| Index (today) | Becomes (at the nullable-company relaxation) |
|---|---|
| `parties (company_id, party_code)` where deleted_at IS NULL | `(COALESCE(company_id, nil-uuid), party_code)` |
| `parties (company_id, npwp)` where npwp IS NOT NULL AND deleted_at IS NULL | `(COALESCE(company_id, nil-uuid), npwp)` |
| `parties (company_id, nik)` where nik IS NOT NULL AND deleted_at IS NULL | `(COALESCE(company_id, nil-uuid), nik)` |

Without the swap, the first shared row lands and every SECOND shared identity with the
same code/NPWP/NIK silently duplicates — the shared catalogue degrades into per-insert
noise. With the swap, the invariant is: one shared row per business key, plus one row per
real company per business key.

The child tables (`party_addresses`, `party_contacts`, `party_emails`, `party_phones`)
have no cross-tenant business keys (their uniqueness is per-parent), so the recipe does
not apply there.

## Where it deliberately does NOT apply

- **Company-private data** (a future party write-path table scoped strictly per company)
  wants plain per-company uniques — folding a sentinel tenant into its key would add a
  meaningless bucket.
- **Reference/enum tables** without `company_id` at all: they have no tenant column to
  fold; global uniques are already correct for them.
