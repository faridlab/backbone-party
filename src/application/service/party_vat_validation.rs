//! Cross-border VAT number validation for the party write path (hand-authored, user-owned).
//!
//! Fail-closed at the OWNER's write path: a VAT number whose country prefix has no known
//! format is REFUSED — the inverse of Odoo's `base_vat` posture, which accepts unknown
//! countries after only a generic alphanumeric check. Refusing loudly (typed error + a
//! `warn!` log naming the country) is the safer default for an Indonesia-first deployment:
//! a wrong VAT number silently accepted breaks e-invoicing and cross-border tax reporting
//! downstream, where it is far more expensive to discover.
//!
//! ## No-VAT sentinel
//!
//! The exact value `/` means "this party deliberately has no VAT number" (the long-standing
//! bookkeeping convention Odoo ports). It is accepted as-is, stored verbatim, and never
//! country-validated. An empty/blank value is treated as "no value" (stored NULL) by the
//! write service — the sentinel exists for the explicit case.
//!
//! ## Semantics ported (offline structural validation)
//!
//! - The EU member-state format table (per-country body shape after the two-letter prefix)
//!   — the offline table Odoo's `base_vat` uses when VIES is unreachable.
//! - The GR→EL alias: Greek VAT numbers use the `EL` prefix although the ISO country code
//!   is `GR`; a `GR`-prefixed number is validated under the Greek (`EL`) body format.
//! - The Belgian mod-97 checksum: the `BE` format carries its own check digits
//!   (`97 - (first 8 digits mod 97)`), verified offline.
//!
//! ## Semantics deliberately NOT ported
//!
//! - **VIES online verification is FENCED**: no network calls from this module. Re-entry
//!   is only via `backbone-integrations` with real billing behind it. Offline validation
//!   here is structural; authoritative registration checks ride that fence.
//! - The non-EU per-country validators (MX/NZ/CO/IN/CN/GT/PE/RU/…) are not ported: those
//!   country prefixes are unknown here and therefore REFUSED. Adding a country means adding
//!   its reviewed format row — a named increment, never a silent generic pass.
//! - The GR/GT test-VAT allowlists Odoo hardcodes (numbers VIES historically mis-reported)
//!   are TEST-FIXTURE-ONLY by ruling: nothing in this production file special-cases them.
//!   Offline structural validation needs no VIES exceptions — tests use format-valid
//!   numbers directly.
//! - Country inference from the partner's address is not ported: the country must be carried
//!   by the number's own two-letter prefix (Odoo falls back to `partner.country_id`, which
//!   fails open when both are missing; we refuse instead).
//!
//! ## Named explicit configuration escape
//!
//! `VatValidationPolicy::allow_unknown_countries()` (wire via
//! `PartyWriteService::with_vat_policy`, or set the environment variable
//! `PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES=1` consumed at service construction) accepts
//! unknown-country numbers with a per-write `warn!` log. The default is fail-closed; the
//! escape is never implicit and logs every time it fires.

use std::fmt;

/// The no-VAT sentinel: "this party deliberately has no VAT number".
pub const NO_VAT_SENTINEL: &str = "/";

/// Environment variable that arms the named unknown-country escape at service construction.
pub const ALLOW_UNKNOWN_COUNTRIES_ENV: &str = "PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES";

/// Why a VAT number was refused. Machine-codeable so the write path can distinguish
/// "unknown country" (the fail-closed posture — maybe an operator escape is warranted)
/// from "known country, malformed number" (never escaped, fix the value).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VatError {
    /// The number carries no two-letter country prefix (we do not infer from addresses).
    NoCountryPrefix,
    /// The country prefix has no reviewed format in the table — refused fail-closed.
    UnknownCountry(String),
    /// The country is known but the number's shape is wrong for it.
    InvalidFormat { country: String, reason: &'static str },
    /// The country is known, the shape fits, but the embedded checksum fails (BE mod-97).
    InvalidChecksum { country: String },
}

impl VatError {
    pub fn code(&self) -> &'static str {
        match self {
            VatError::NoCountryPrefix => "vat_no_country_prefix",
            VatError::UnknownCountry(_) => "vat_unknown_country",
            VatError::InvalidFormat { .. } => "vat_invalid_format",
            VatError::InvalidChecksum { .. } => "vat_invalid_checksum",
        }
    }
    /// The two-letter country the refusal names, when it carries one (for the loud log).
    pub fn country(&self) -> Option<&str> {
        match self {
            VatError::UnknownCountry(c) => Some(c),
            VatError::InvalidFormat { country, .. } | VatError::InvalidChecksum { country } => {
                Some(country)
            }
            VatError::NoCountryPrefix => None,
        }
    }
}

impl fmt::Display for VatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VatError::NoCountryPrefix => {
                write!(f, "vat_no_country_prefix: VAT numbers must start with their two-letter country code")
            }
            VatError::UnknownCountry(c) => write!(
                f,
                "vat_unknown_country: {c} has no reviewed VAT format (fail-closed; the escape is {ALLOW_UNKNOWN_COUNTRIES_ENV} or VatValidationPolicy::allow_unknown_countries)"
            ),
            VatError::InvalidFormat { country, reason } => {
                write!(f, "vat_invalid_format: not a valid {country} VAT number: {reason}")
            }
            VatError::InvalidChecksum { country } => write!(
                f,
                "vat_invalid_checksum: {country} check digits do not match the number body"
            ),
        }
    }
}
impl std::error::Error for VatError {}

/// Validation posture for the party write path. The default is fail-closed for unknown
/// countries; the escape is a named, explicit choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VatValidationPolicy {
    /// The NAMED ESCAPE: accept VAT numbers whose country prefix has no reviewed format.
    /// Every acceptance under the escape logs a warning naming the country.
    pub allow_unknown_countries: bool,
}

impl VatValidationPolicy {
    /// The default posture: unknown countries are refused loudly.
    pub const FAIL_CLOSED: Self = Self { allow_unknown_countries: false };

    /// The NAMED EXPLICIT ESCAPE: unknown-country numbers are accepted with a per-write
    /// warning. Opt in via configuration, never by default.
    pub const ALLOW_UNKNOWN_COUNTRIES: Self = Self { allow_unknown_countries: true };

    /// Read the policy from the environment (`PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES=1|true`
    /// arms the escape). Unset or any other value stays fail-closed — the escape is never
    /// implicit.
    pub fn from_env() -> Self {
        let armed = std::env::var(ALLOW_UNKNOWN_COUNTRIES_ENV)
            .ok()
            .map(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "True"))
            .unwrap_or(false);
        if armed {
            tracing::warn!(
                env_var = ALLOW_UNKNOWN_COUNTRIES_ENV,
                "party VAT validation escape ARMED: unknown-country VAT numbers will be accepted with a per-write warning"
            );
            Self::ALLOW_UNKNOWN_COUNTRIES
        } else {
            Self::FAIL_CLOSED
        }
    }
}

impl Default for VatValidationPolicy {
    fn default() -> Self {
        Self::FAIL_CLOSED
    }
}

/// Canonical form for storage: uppercase, no spaces, dots, or dashes.
/// Purely formatting — never changes the country prefix or digits.
pub fn normalize_vat(v: &str) -> String {
    v.trim()
        .to_uppercase()
        .chars()
        .filter(|c| !matches!(c, ' ' | '.' | '-'))
        .collect()
}

// --- per-country body validators (the part of the number after the two-letter prefix) ---
// Each returns Some(reason) when the body is wrong for that country. Static, table-driven,
// no regex dependency: char-class + length checks are enough for every row.

fn digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
fn alnum_upper(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

/// A country row's body check: returns Some(reason) when the body (after the two-letter
/// prefix) is wrong for that country.
type VatFormatFn = fn(&str) -> Option<&'static str>;

/// EU member-state format table: body shape after the two-letter prefix.
/// Adding a country = adding a reviewed row here (never a generic fallback).
const VAT_COUNTRY_FORMATS: &[(&str, VatFormatFn)] = &[
    // AT: U + 8 digits.
    ("AT", |b| {
        if b.len() == 9 && b.starts_with('U') && digits(&b[1..]) { None } else { Some("expected U followed by 8 digits") }
    }),
    // BE: 10 digits starting 0/1, with mod-97 check digits (verified separately).
    ("BE", |b| {
        if b.len() == 10 && digits(b) && (b.starts_with('0') || b.starts_with('1')) { None } else { Some("expected 10 digits starting with 0 or 1") }
    }),
    // BG: 9 or 10 digits.
    ("BG", |b| {
        if (b.len() == 9 || b.len() == 10) && digits(b) { None } else { Some("expected 9 or 10 digits") }
    }),
    // CY: 8 digits + 1 letter.
    ("CY", |b| {
        if b.len() == 9 && digits(&b[..8]) && b[8..].chars().all(|c| c.is_ascii_uppercase()) { None } else { Some("expected 8 digits followed by a letter") }
    }),
    // CZ: 8-10 digits.
    ("CZ", |b| {
        if (8..=10).contains(&b.len()) && digits(b) { None } else { Some("expected 8 to 10 digits") }
    }),
    // DE: 9 digits.
    ("DE", |b| {
        if b.len() == 9 && digits(b) { None } else { Some("expected 9 digits") }
    }),
    // DK: 8 digits.
    ("DK", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // EE: 9 digits.
    ("EE", |b| {
        if b.len() == 9 && digits(b) { None } else { Some("expected 9 digits") }
    }),
    // EL: 9 digits (Greece issues VAT under EL, not its ISO code GR).
    ("EL", |b| {
        if b.len() == 9 && digits(b) { None } else { Some("expected 9 digits") }
    }),
    // ES: 1 alnum + 7 digits + 1 alnum.
    ("ES", |b| {
        if b.len() == 9 && alnum_upper(&b[..1]) && digits(&b[1..8]) && alnum_upper(&b[8..]) { None } else { Some("expected letter/digit, 7 digits, letter/digit") }
    }),
    // FI: 8 digits.
    ("FI", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // FR: 2 alphanumerics + 9 digits.
    ("FR", |b| {
        if b.len() == 11 && alnum_upper(&b[..2]) && digits(&b[2..]) { None } else { Some("expected 2 alphanumeric characters followed by 9 digits") }
    }),
    // HR: 11 digits.
    ("HR", |b| {
        if b.len() == 11 && digits(b) { None } else { Some("expected 11 digits") }
    }),
    // HU: 8 digits.
    ("HU", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // IE: 7 alphanumerics + 1 letter, optionally + 1 more letter.
    ("IE", |b| {
        let ok = (b.len() == 8 || b.len() == 9)
            && alnum_upper(&b[..7])
            && b[7..8].chars().all(|c| c.is_ascii_uppercase())
            && (b.len() == 8 || b[8..9].chars().all(|c| c.is_ascii_uppercase()));
        if ok { None } else { Some("expected 7 alphanumeric characters and 1-2 letters") }
    }),
    // IT: 11 digits.
    ("IT", |b| {
        if b.len() == 11 && digits(b) { None } else { Some("expected 11 digits") }
    }),
    // LT: 9 or 12 digits.
    ("LT", |b| {
        if (b.len() == 9 || b.len() == 12) && digits(b) { None } else { Some("expected 9 or 12 digits") }
    }),
    // LU: 8 digits.
    ("LU", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // LV: 11 digits.
    ("LV", |b| {
        if b.len() == 11 && digits(b) { None } else { Some("expected 11 digits") }
    }),
    // MT: 8 digits.
    ("MT", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // NL: 9 digits + B + 2 digits.
    ("NL", |b| {
        let ok = b.len() == 12 && digits(&b[..9]) && b.starts_with(|c: char| c.is_ascii_digit())
            && b.as_bytes().get(9) == Some(&b'B');
        let ok = ok && digits(&b[10..]);
        if ok { None } else { Some("expected 9 digits, the letter B, and 2 digits") }
    }),
    // PL: 10 digits.
    ("PL", |b| {
        if b.len() == 10 && digits(b) { None } else { Some("expected 10 digits") }
    }),
    // PT: 9 digits.
    ("PT", |b| {
        if b.len() == 9 && digits(b) { None } else { Some("expected 9 digits") }
    }),
    // RO: 2-10 digits.
    ("RO", |b| {
        if (2..=10).contains(&b.len()) && digits(b) { None } else { Some("expected 2 to 10 digits") }
    }),
    // SE: 10 digits + 01.
    ("SE", |b| {
        if b.len() == 12 && digits(&b[..10]) && &b[10..] == "01" { None } else { Some("expected 10 digits followed by 01") }
    }),
    // SI: 8 digits.
    ("SI", |b| {
        if b.len() == 8 && digits(b) { None } else { Some("expected 8 digits") }
    }),
    // SK: 10 digits.
    ("SK", |b| {
        if b.len() == 10 && digits(b) { None } else { Some("expected 10 digits") }
    }),
];

/// Country-prefix aliases: VAT prefixes that differ from the format-table key.
/// Greece issues VAT numbers under `EL`; a `GR`-prefixed number is validated as Greek.
const VAT_COUNTRY_ALIASES: &[(&str, &str)] = &[("GR", "EL")];

fn canonical_country(prefix: &str) -> &str {
    for (from, to) in VAT_COUNTRY_ALIASES {
        if *from == prefix {
            return to;
        }
    }
    prefix
}

/// Belgian VAT check digits: valid iff the last two digits equal `97 - (first 8 mod 97)`
/// (with `97 - 0 = 97`). ISO 7064-style redundancy carried by the number itself.
fn belgium_checksum_ok(body_10_digits: &str) -> bool {
    let digits: Vec<u32> = body_10_digits.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != 10 {
        return false;
    }
    let head: u32 = digits[..8].iter().fold(0u32, |acc, d| acc * 10 + d);
    let expected = 97 - (head % 97);
    let actual = digits[8] * 10 + digits[9];
    expected == actual
}

/// Offline structural VAT validation, fail-closed. Returns the canonical (normalized)
/// form for storage. The `/` no-VAT sentinel passes through unchanged.
///
/// This is the policy-free core; the write path calls [`validate_vat_with`].
pub fn validate_vat(raw: &str) -> Result<String, VatError> {
    let v = normalize_vat(raw);
    if v == NO_VAT_SENTINEL {
        return Ok(v);
    }
    if v.len() < 3 || !v[..2].chars().all(|c| c.is_ascii_uppercase()) {
        return Err(VatError::NoCountryPrefix);
    }
    let prefix = &v[..2];
    let country = canonical_country(prefix);
    let body = &v[2..];
    let check = VAT_COUNTRY_FORMATS.iter().find(|(c, _)| *c == country).map(|(_, f)| f);
    let check = match check {
        Some(f) => f,
        None => return Err(VatError::UnknownCountry(prefix.to_string())),
    };
    if let Some(reason) = check(body) {
        return Err(VatError::InvalidFormat { country: country.to_string(), reason });
    }
    // Checksums the format itself carries (everything else rides the VIES fence).
    if country == "BE" && !belgium_checksum_ok(body) {
        return Err(VatError::InvalidChecksum { country: country.to_string() });
    }
    Ok(v)
}

/// Policy-aware validation for the party write path. Unknown countries are refused
/// loudly (warn log) unless the NAMED ESCAPE is armed — in which case they are
/// accepted with a warning naming the country, so the escape is auditable in logs.
/// Format and checksum failures are NEVER escaped.
pub fn validate_vat_with(policy: &VatValidationPolicy, raw: &str) -> Result<String, VatError> {
    match validate_vat(raw) {
        Ok(v) => Ok(v),
        Err(VatError::UnknownCountry(country)) if policy.allow_unknown_countries => {
            tracing::warn!(
                country = %country,
                "party VAT validation escape: accepting a VAT number whose country has no reviewed format"
            );
            Ok(normalize_vat(raw))
        }
        Err(e) => {
            tracing::warn!(error = %e.code(), "party VAT validation refused a value: {e}");
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_eu_numbers() {
        assert_eq!(validate_vat("DE123456789").unwrap(), "DE123456789");
        assert_eq!(validate_vat("fr40303265045").unwrap(), "FR40303265045");
        assert_eq!(validate_vat("NL 0044.95544B01").unwrap(), "NL004495544B01");
        assert_eq!(validate_vat("IE6388047V").unwrap(), "IE6388047V");
    }

    #[test]
    fn greek_numbers_validate_under_the_el_format() {
        assert_eq!(validate_vat("EL123456789").unwrap(), "EL123456789");
        assert_eq!(validate_vat("GR123456789").unwrap(), "GR123456789");
        assert!(matches!(validate_vat("EL12345"), Err(VatError::InvalidFormat { .. })));
    }

    #[test]
    fn belgian_mod97_checksum_enforced() {
        assert_eq!(validate_vat("BE0428759497").unwrap(), "BE0428759497");
        assert!(matches!(validate_vat("BE0428759498"), Err(VatError::InvalidChecksum { .. })));
    }

    #[test]
    fn unknown_country_refused_fail_closed() {
        assert_eq!(validate_vat("XX123456789").unwrap_err(), VatError::UnknownCountry("XX".into()));
        // Non-EU countries Odoo has per-country validators for are unknown here, too.
        assert!(matches!(validate_vat("GT1234567K"), Err(VatError::UnknownCountry(_))));
    }

    #[test]
    fn missing_prefix_refused() {
        assert_eq!(validate_vat("123456789").unwrap_err(), VatError::NoCountryPrefix);
        assert_eq!(validate_vat("1234567890AB").unwrap_err(), VatError::NoCountryPrefix);
    }

    #[test]
    fn sentinel_and_normalization() {
        assert_eq!(validate_vat("/").unwrap(), "/");
        assert_eq!(validate_vat(" / ").unwrap(), "/");
        assert_eq!(validate_vat("de-123.456 789").unwrap(), "DE123456789");
    }

    #[test]
    fn malformed_known_country_refused() {
        assert!(matches!(validate_vat("DE123"), Err(VatError::InvalidFormat { country, .. }) if country == "DE"));
        assert!(matches!(validate_vat("SE1234567890"), Err(VatError::InvalidFormat { .. })));
    }

    #[test]
    fn escape_accepts_unknown_but_never_malformed() {
        let escape = VatValidationPolicy::ALLOW_UNKNOWN_COUNTRIES;
        assert_eq!(validate_vat_with(&escape, "XX123456789").unwrap(), "XX123456789");
        assert!(matches!(
            validate_vat_with(&escape, "DE123"),
            Err(VatError::InvalidFormat { .. })
        ));
        assert!(matches!(
            validate_vat_with(&VatValidationPolicy::FAIL_CLOSED, "XX123456789"),
            Err(VatError::UnknownCountry(_))
        ));
    }
}
