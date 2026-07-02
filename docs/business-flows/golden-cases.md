# Party — Golden Cases (the oracle)

Exact expected results, mirroring `tests/party_golden_cases.rs` and `tests/integrity_probes.rs`.

## Validated writes (`tests/party_golden_cases.rs`)

| Case | Input | Expected |
|------|-------|----------|
| **PGC-1** | create org party `{partyCode, name, npwp(15)}` | `201`; `party_kind=organization`, `status=active`. |
| **PGC-2** | npwp `"123"`, then person nik `"12345"` | `422 invalid_npwp`, then `422 invalid_nik`; nothing written. |
| **PGC-3** | duplicate party_code, then duplicate npwp | `422 duplicate_party_code`, then `422 duplicate_npwp`. |
| **PGC-4** | address/email/phone with a non-existent partyId | `422 party_not_found` each. |
| **PGC-5** | address with a geo `subdistrictId` | `201`; the geo id is stored **verbatim** (opaque logical FK, no cross-module join). |
| **PGC-6** (unit) | NPWP/NIK validators | NPWP accepts 15/16 digits; NIK accepts exactly 16. |

## Guarded write path (`tests/integrity_probes.rs`)

| Case | Input via guarded routes | Expected |
|------|--------------------------|----------|
| **IGC-1** | `POST /parties/bulk` (generic) | `405/404` — parties only via the validated path. |
| **IGC-2** | `POST /parties` with `npwp:"123"` | `422`. |
| **IGC-3** | `POST /party-addresses` with a missing party | `422`. |
| **IGC-4** | valid party + geo-linked address | both `201`. |
| **IGC-5** | second `isPrimary:true` address for a party | `422 duplicate_primary`; a non-primary is still `201`. |
| **IGC-6** | `person` with no name parts; `organization` carrying a NIK | both `422 inconsistent_party_kind`. |
| **IGC-7** | `POST /party-set-primary` promoting a second address | `200`; exactly one primary, the promoted one. |

## Conventions
- New party: `status=active`, `party_kind=organization` unless overridden.
- Uniqueness (`party_code`, `npwp`, `nik`) is partial on not-deleted rows.
- Address geo ids (`country/province/city/district/subdistrict_id`) are logical FKs to `backbone-geo`
  — stored as opaque UUIDs, validated at the ACL/consumer layer.
