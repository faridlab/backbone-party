# VAT number validation (party write path)

The `parties.vat` column carries a cross-border VAT number (a country-prefixed identifier
such as `DE123456789` or `FR40303265045`). It is validated **fail-closed at the owner's
write path** — `PartyWriteService::create_party` (see
`src/application/service/party_vat_validation.rs`). This is the deliberate inverse of
Odoo's `base_vat`, which accepts unknown countries after only a generic alphanumeric
check: here, a value whose country prefix has no reviewed format is **refused**, loudly.

Indonesia-first context: the statutory identifiers for Indonesian parties are `npwp` /
`nik`, which have their own checks on the same write path. The `vat` field exists for
cross-border counterparties.

## Behaviour

| Input | Outcome |
|---|---|
| `DE123456789`, `fr40303265045`, `NL 0044.95544B01` | accepted, stored **canonicalized** (uppercased; spaces, dots, dashes stripped) |
| `BE0428759497` | accepted (Belgian mod-97 checksum passes) |
| `BE0428759498` | refused — `vat_invalid_checksum` (one check digit off) |
| `XX123456789` | refused — `vat_unknown_country` (the fail-closed posture) |
| `123456789` (no prefix) | refused — `vat_no_country_prefix` (country is never inferred from addresses) |
| `/` | **accepted verbatim** — the no-VAT sentinel (below) |
| empty / whitespace | treated as "no value" — stored `NULL` |

Every refusal logs a `warn!` naming the reason (and the country, when one is claimed).
Unknown-country refusals carry their own error code (`vat_unknown_country`, HTTP 422) so
operators can distinguish "the escape may be warranted" from "the number is malformed" —
format and checksum failures are never escape-able.

## The `/` no-VAT sentinel

The exact value `/` means "this party deliberately has no VAT number" — the long-standing
bookkeeping convention Odoo ports. It is accepted without country validation and stored
verbatim, so a "no VAT" answer stays distinguishable from "VAT not asked for" (`NULL`).
Blanks do not carry that meaning: an empty value stores `NULL`.

## What was ported (offline structural validation)

- **EU member-state format table** — per-country body shapes after the two-letter prefix
  (AT, BE, BG, CY, CZ, DE, DK, EE, EL, ES, FI, FR, HR, HU, IE, IT, LT, LU, LV, MT, NL,
  PL, PT, RO, SE, SI, SK), the offline table Odoo's `base_vat` uses when VIES is
  unreachable.
- **The GR→EL alias** — Greece issues VAT numbers under `EL` although its ISO code is
  `GR`; a `GR`-prefixed number is validated under the Greek (`EL`) format (kept verbatim).
- **The Belgian mod-97 checksum** — the `BE` format embeds check digits
  (`97 − (first 8 digits mod 97)`), verified offline.

## What was deliberately NOT ported

- **VIES online verification stays FENCED.** No network calls from this module. Re-entry
  is only via `backbone-integrations` with real billing behind it; offline validation
  here is structural, and authoritative registration checks ride that fence. This fence
  is deliberate and recorded — do not "fix" it by adding an HTTP client here.
- **Non-EU per-country validators** (MX/NZ/CO/IN/CN/GT/PE/RU/…) are not ported; those
  prefixes are refused as unknown countries. Adding a country is a reviewed table row in
  `party_vat_validation.rs` — never a generic pass-through.
- **The GR/GT test-VAT allowlists** Odoo hardcodes (numbers VIES historically
  mis-reported) are TEST-FIXTURE-ONLY: nothing in production code special-cases them, and
  nothing needs to — offline structural validation accepts format-valid numbers directly.
- **Country inference from the party's address** is not ported: the country must ride the
  number's own prefix (Odoo's fallback to `partner.country_id` fails open when both are
  missing; we refuse instead).
- **UK (`GB`/`XI`) is not in the table** — post-Brexit UK VAT is outside the EU format
  table Odoo ports; adding it is a named row if ever needed.

## The named configuration escape

`PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES=1` (or `VatValidationPolicy::ALLOW_UNKNOWN_COUNTRIES`
wired via `PartyWriteService::with_vat_policy`) accepts unknown-country VAT numbers. The
escape:

- is **never implicit** — the default is fail-closed; arming it logs a `warn!` at service
  construction and a `warn!` per accepted unknown-country value (auditable in logs);
- **never widens correctness** — malformed known-country numbers and checksum failures
  are still refused under the escape.

`PartyModule::builder()` applies `VatValidationPolicy::from_env()` when it constructs the
module's write service, so the environment variable is honored in the standard composed
host; hosts building the service directly choose the posture explicitly.

## Probes

`tests/vat_probes.rs` proves the posture end-to-end against a migrated database: valid
number accepted + canonicalized; Belgian checksum enforced with nothing written on
refusal; unknown country refused with the distinct code and no row; the `/` sentinel
stored verbatim; the escape honored (and still refusing malformed numbers); malformed and
prefix-less values refused. Unit tests in `party_vat_validation.rs` cover the format
table per country.
