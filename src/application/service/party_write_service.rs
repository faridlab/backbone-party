//! Validated write path for Party + its multi-channel children — hand-authored (user-owned).
//!
//! Closes the CRUD-bypass: the generated 12-endpoint CRUD writes rows with NO domain validation.
//! Here `create_party` validates NPWP/NIK format + uniqueness; the child writers verify the party
//! exists. Geo ids on an address are LOGICAL FKs (validated at the ACL layer / consuming service,
//! not against geo's schema here — keeps party decoupled from geo).
//!
//! Tenancy: none, by design (ADR-0029). The module is tenant-agnostic — no tenant key on any
//! write, no scope binding inside this service. The COMPOSING service owns the posture: when it
//! mounts these routes under an auth middleware that binds a row scope (e.g.
//! `with_org_request_scope`), the database fence owns tenant isolation; a deployment that mounts
//! them unfenced gets an unfenced module.
//!
//! SQL lives in the repositories (`PartyRepository`, `PartyAddressRepository`, …), not here, per
//! the module's 4-layer rule. This service only orchestrates validation + dispatch + the
//! duplicate-key → typed-error mapping.

use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::application::service::party_vat_validation::{validate_vat_with, VatError, VatValidationPolicy};

use crate::infrastructure::persistence::{
    NewPartyAddressRow, NewPartyContactRow, NewPartyEmailRow, NewPartyPhoneRow, NewPartyRow,
    PartyAddressRepository, PartyContactRepository, PartyEmailRepository, PartyPhoneRepository,
    PartyRepository,
};

#[derive(Debug)]
pub enum PartyWriteError {
    PartyNotFound(Uuid),
    DuplicateCode(String),
    DuplicateNpwp(String),
    DuplicateNik(String),
    InvalidNpwp(String),
    InvalidNik(String),
    /// A VAT number whose country is KNOWN but whose shape/checksum is wrong.
    InvalidVat(String),
    /// A VAT number whose country prefix has no reviewed format — refused fail-closed
    /// (distinct from InvalidVat so operators can see the escape may be the answer).
    VatUnknownCountry(String),
    InvalidEmail(String),
    /// A party_kind/field mismatch (e.g. person with no name parts, org carrying a NIK).
    InconsistentKind(String),
    /// The party already has a primary of this kind (one-primary-per-party invariant).
    DuplicatePrimary(&'static str),
    Db(sqlx::Error),
}

impl PartyWriteError {
    pub fn code(&self) -> &'static str {
        match self {
            PartyWriteError::PartyNotFound(_) => "party_not_found",
            PartyWriteError::DuplicateCode(_) => "duplicate_party_code",
            PartyWriteError::DuplicateNpwp(_) => "duplicate_npwp",
            PartyWriteError::DuplicateNik(_) => "duplicate_nik",
            PartyWriteError::InvalidNpwp(_) => "invalid_npwp",
            PartyWriteError::InvalidNik(_) => "invalid_nik",
            PartyWriteError::InvalidVat(_) => "invalid_vat",
            PartyWriteError::VatUnknownCountry(_) => "vat_unknown_country",
            PartyWriteError::InvalidEmail(_) => "invalid_email",
            PartyWriteError::InconsistentKind(_) => "inconsistent_party_kind",
            PartyWriteError::DuplicatePrimary(_) => "duplicate_primary",
            PartyWriteError::Db(_) => "internal_error",
        }
    }
    pub fn http_status(&self) -> u16 {
        match self {
            PartyWriteError::Db(_) => 500,
            _ => 422,
        }
    }
}
impl std::fmt::Display for PartyWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())?;
        match self {
            PartyWriteError::PartyNotFound(id) => write!(f, ": {id}"),
            PartyWriteError::DuplicateCode(v)
            | PartyWriteError::DuplicateNpwp(v)
            | PartyWriteError::DuplicateNik(v)
            | PartyWriteError::InvalidNpwp(v)
            | PartyWriteError::InvalidNik(v)
            | PartyWriteError::InvalidVat(v)
            | PartyWriteError::VatUnknownCountry(v)
            | PartyWriteError::InvalidEmail(v)
            | PartyWriteError::InconsistentKind(v) => write!(f, ": {v}"),
            PartyWriteError::DuplicatePrimary(kind) => write!(f, ": {kind}"),
            PartyWriteError::Db(_) => Ok(()),
        }
    }
}
impl std::error::Error for PartyWriteError {}
impl From<sqlx::Error> for PartyWriteError {
    fn from(e: sqlx::Error) -> Self {
        PartyWriteError::Db(e)
    }
}

/// Indonesian NPWP: 15 (legacy) or 16 (NIK-based) digits, ignoring separators.
pub fn validate_npwp(v: &str) -> bool {
    let d = v.chars().filter(|c| c.is_ascii_digit()).count();
    d == 15 || d == 16
}
/// Indonesian NIK (national ID): exactly 16 digits.
pub fn validate_nik(v: &str) -> bool {
    v.chars().filter(|c| c.is_ascii_digit()).count() == 16
}

#[derive(Debug, Clone)]
pub struct NewParty {
    pub party_code: String,
    pub party_kind: Option<String>,
    pub name: String,
    pub legal_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub npwp: Option<String>,
    pub nik: Option<String>,
    /// Cross-border VAT number. Validated fail-closed; '/' is the no-VAT sentinel.
    /// Stored in canonical form (uppercased, separators stripped) or verbatim when '/'.
    pub vat: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NewAddress {
    pub party_id: Uuid,
    pub address_type: Option<String>,
    pub label: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub country_id: Option<Uuid>,
    pub province_id: Option<Uuid>,
    pub city_id: Option<Uuid>,
    pub district_id: Option<Uuid>,
    pub subdistrict_id: Option<Uuid>,
    pub postal_code: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub is_primary: bool,
    pub is_billing: bool,
    pub is_shipping: bool,
}

#[derive(Debug, Clone)]
pub struct NewContact {
    pub party_id: Uuid,
    pub name: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct NewEmail {
    pub party_id: Uuid,
    pub label: Option<String>,
    pub email: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct NewPhone {
    pub party_id: Uuid,
    pub label: Option<String>,
    pub phone: String,
    pub is_primary: bool,
}

#[derive(Clone)]
pub struct PartyWriteService {
    db_pool: PgPool,
    /// VAT validation posture. Default fail-closed for unknown countries; the named
    /// escape is wired explicitly (see `VatValidationPolicy`).
    vat_policy: crate::application::service::party_vat_validation::VatValidationPolicy,
}

impl PartyWriteService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool, vat_policy: VatValidationPolicy::FAIL_CLOSED }
    }

    /// The pool this service was built with — the composing app's boot pool.
    /// Handlers use it as the fallback for callers that carry no
    /// tenant-dedicated pool on the request.
    pub fn pool(&self) -> &PgPool {
        &self.db_pool
    }

    /// Explicit VAT validation posture for this service instance. Hosts wiring the named
    /// escape pass `VatValidationPolicy::ALLOW_UNKNOWN_COUNTRIES` (or
    /// `VatValidationPolicy::from_env()` to honor `PARTY_VAT_ALLOW_UNKNOWN_COUNTRIES`);
    /// the module builder already applies `from_env()`.
    pub fn with_vat_policy(
        db_pool: PgPool,
        vat_policy: crate::application::service::party_vat_validation::VatValidationPolicy,
    ) -> Self {
        Self { db_pool, vat_policy }
    }

    fn is_dup(e: &sqlx::Error, needle: &str) -> bool {
        e.as_database_error()
            .map(|d| d.is_unique_violation() && d.constraint().unwrap_or("").contains(needle))
            .unwrap_or(false)
    }
    fn is_unique(e: &sqlx::Error) -> bool {
        e.as_database_error().map(|d| d.is_unique_violation()).unwrap_or(false)
    }

    /// Existence check. Under a composing service's row fence (RLS), the scope bound on the
    /// request connection limits what is visible; with no fence mounted, this is a plain lookup.
    async fn party_exists(&self, id: Uuid) -> Result<bool, PartyWriteError> {
        let parties = PartyRepository::new(self.db_pool.clone());
        Ok(parties.find_active_id(&self.db_pool, id).await?.is_some())
    }

    pub async fn create_party(&self, p: NewParty) -> Result<Uuid, PartyWriteError> {
        if let Some(n) = &p.npwp {
            if !validate_npwp(n) {
                return Err(PartyWriteError::InvalidNpwp(n.clone()));
            }
        }
        if let Some(n) = &p.nik {
            if !validate_nik(n) {
                return Err(PartyWriteError::InvalidNik(n.clone()));
            }
        }
        // VAT: fail-closed for unknown countries; '/' is the no-VAT sentinel; an empty
        // value means "no value" (stored NULL). Unknown-country refusals carry a loud
        // warn log from the validator plus a distinct error code so operators can tell
        // the escape-able refusal from a malformed known-country number.
        let vat: Option<String> = match p.vat.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(raw) => match validate_vat_with(&self.vat_policy, raw) {
                Ok(canonical) => Some(canonical),
                Err(VatError::UnknownCountry(country)) => {
                    return Err(PartyWriteError::VatUnknownCountry(country));
                }
                Err(e) => {
                    return Err(PartyWriteError::InvalidVat(format!("{raw} ({e})")));
                }
            },
        };
        let kind = p.party_kind.clone().unwrap_or_else(|| "organization".to_string());
        // Kind/field coherence (council 2026-07-02): a person needs a name part; an organization
        // needs a legal_name and cannot carry a NIK (a person's national ID).
        let has_name = |s: &Option<String>| s.as_deref().map(|v| !v.trim().is_empty()).unwrap_or(false);
        match kind.as_str() {
            "person" => {
                if !has_name(&p.first_name) && !has_name(&p.last_name) {
                    return Err(PartyWriteError::InconsistentKind(
                        "person requires first_name or last_name".into(),
                    ));
                }
            }
            "organization" => {
                if !has_name(&p.legal_name) {
                    return Err(PartyWriteError::InconsistentKind(
                        "organization requires legal_name".into(),
                    ));
                }
                if has_name(&p.nik) {
                    return Err(PartyWriteError::InconsistentKind(
                        "organization cannot carry a NIK (person national ID)".into(),
                    ));
                }
            }
            _ => {}
        }
        let id = Uuid::new_v4();
        let parties = PartyRepository::new(self.db_pool.clone());
        let r = parties.insert_from_new(
            &self.db_pool,
            &NewPartyRow {
                id,
                party_code: &p.party_code,
                party_kind: &kind,
                name: &p.name,
                legal_name: p.legal_name.as_deref(),
                first_name: p.first_name.as_deref(),
                last_name: p.last_name.as_deref(),
                npwp: p.npwp.as_deref(),
                nik: p.nik.as_deref(),
                vat: vat.as_deref(),
            },
        ).await;
        match r {
            Ok(_) => Ok(id),
            Err(e) if Self::is_dup(&e, "npwp") => Err(PartyWriteError::DuplicateNpwp(p.npwp.unwrap_or_default())),
            Err(e) if Self::is_dup(&e, "nik") => Err(PartyWriteError::DuplicateNik(p.nik.unwrap_or_default())),
            Err(e) if Self::is_dup(&e, "party_code") || Self::is_dup(&e, "parties") => {
                Err(PartyWriteError::DuplicateCode(p.party_code))
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn add_address(&self, a: NewAddress) -> Result<Uuid, PartyWriteError> {
        if !self.party_exists(a.party_id).await? {
            return Err(PartyWriteError::PartyNotFound(a.party_id));
        }
        let id = Uuid::new_v4();
        let atype = a.address_type.clone().unwrap_or_else(|| "home".to_string());
        let addresses = PartyAddressRepository::new(self.db_pool.clone());
        let r = addresses.insert_from_new(
            &self.db_pool,
            &NewPartyAddressRow {
                id,
                party_id: a.party_id,
                address_type: &atype,
                label: a.label.as_deref(),
                line1: &a.line1,
                line2: a.line2.as_deref(),
                country_id: a.country_id,
                province_id: a.province_id,
                city_id: a.city_id,
                district_id: a.district_id,
                subdistrict_id: a.subdistrict_id,
                postal_code: a.postal_code.as_deref(),
                latitude: a.latitude,
                longitude: a.longitude,
                is_primary: a.is_primary,
                is_billing: a.is_billing,
                is_shipping: a.is_shipping,
            },
        ).await;
        Self::ok_or_primary(r, id, "address")
    }

    pub async fn add_contact(&self, c: NewContact) -> Result<Uuid, PartyWriteError> {
        if !self.party_exists(c.party_id).await? {
            return Err(PartyWriteError::PartyNotFound(c.party_id));
        }
        let id = Uuid::new_v4();
        let contacts = PartyContactRepository::new(self.db_pool.clone());
        let r = contacts.insert_from_new(
            &self.db_pool,
            &NewPartyContactRow {
                id,
                party_id: c.party_id,
                name: &c.name,
                job_title: c.job_title.as_deref(),
                department: c.department.as_deref(),
                email: c.email.as_deref(),
                phone: c.phone.as_deref(),
                is_primary: c.is_primary,
            },
        ).await;
        Self::ok_or_primary(r, id, "contact")
    }

    pub async fn add_email(&self, e: NewEmail) -> Result<Uuid, PartyWriteError> {
        if !self.party_exists(e.party_id).await? {
            return Err(PartyWriteError::PartyNotFound(e.party_id));
        }
        if !e.email.contains('@') {
            return Err(PartyWriteError::InvalidEmail(e.email));
        }
        let id = Uuid::new_v4();
        let label = e.label.clone().unwrap_or_else(|| "main".to_string());
        let emails = PartyEmailRepository::new(self.db_pool.clone());
        let r = emails.insert_from_new(
            &self.db_pool,
            &NewPartyEmailRow {
                id,
                party_id: e.party_id,
                label: &label,
                email: &e.email,
                is_primary: e.is_primary,
            },
        ).await;
        Self::ok_or_primary(r, id, "email")
    }

    pub async fn add_phone(&self, p: NewPhone) -> Result<Uuid, PartyWriteError> {
        if !self.party_exists(p.party_id).await? {
            return Err(PartyWriteError::PartyNotFound(p.party_id));
        }
        let id = Uuid::new_v4();
        let label = p.label.clone().unwrap_or_else(|| "mobile".to_string());
        let phones = PartyPhoneRepository::new(self.db_pool.clone());
        let r = phones.insert_from_new(
            &self.db_pool,
            &NewPartyPhoneRow {
                id,
                party_id: p.party_id,
                label: &label,
                phone: &p.phone,
                is_primary: p.is_primary,
            },
        ).await;
        Self::ok_or_primary(r, id, "phone")
    }

    fn ok_or_primary(
        r: Result<(), sqlx::Error>,
        id: Uuid,
        kind: &'static str,
    ) -> Result<Uuid, PartyWriteError> {
        match r {
            Ok(_) => Ok(id),
            Err(e) if Self::is_unique(&e) => Err(PartyWriteError::DuplicatePrimary(kind)),
            Err(e) => Err(e.into()),
        }
    }

    /// Switch which child of a kind is primary: clears is_primary on all of the party's children
    /// of that kind, then sets it on `child_id` — in one transaction (keeps the one-primary
    /// invariant switchable, since the guarded surface is otherwise create-only).
    ///
    /// Dispatches on `kind` to the per-child repository's `clear_primary_for_party` +
    /// `set_primary_child` methods, killing the old `format!("UPDATE party.{table} …")` smell —
    /// each repo knows its own table at compile time.
    pub async fn set_primary(
        &self,
        pool: &PgPool,
        party_id: Uuid,
        kind: &str,
        child_id: Uuid,
    ) -> Result<(), PartyWriteError> {
        // Validate kind BEFORE opening the tx so unknown kinds bail with no side effects.
        match kind {
            "address" | "contact" | "email" | "phone" => {}
            _ => return Err(PartyWriteError::InconsistentKind(format!("unknown child kind: {kind}"))),
        }
        if !self.party_exists(party_id).await? {
            return Err(PartyWriteError::PartyNotFound(party_id));
        }
        // The caller names the pool this write belongs to: under a tenant
        // router the handler passes the request's tenant-dedicated pool (the
        // service's own pool is the composing app's boot pool — the wrong
        // database for any other tenant). Unfenced deployments pass their own.
        let mut tx = pool.begin().await?;
        // Propagate the ambient request scope, when one is bound, onto this
        // transaction: the repositories' execute_scoped helpers ride the
        // request-dedicated connection, but this pool transaction does not, and
        // rows a deployment's fence decorates are invisible to an unscoped
        // connection. Binding the ambient scope relay-only keeps the module
        // posture-agnostic — unfenced deployments have no ambient scope and
        // skip this entirely.
        if let Some(scope) = backbone_orm::org_scope::current_org_scope() {
            backbone_orm::org_scope::bind_org_scope_on(&mut *tx, &scope).await?;
        }
        // Clear first (so the partial-unique index never sees two primaries mid-transaction).
        // Dispatch to the per-child repo so the table name is a compile-time constant, not a
        // string-built identifier.
        let n = match kind {
            "address" => {
                let repo = PartyAddressRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id).await?
            }
            "contact" => {
                let repo = PartyContactRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id).await?
            }
            "email" => {
                let repo = PartyEmailRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id).await?
            }
            "phone" => {
                let repo = PartyPhoneRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id).await?
            }
            // Unreachable: validated above. Match kept exhaustive so adding a new kind without
            // wiring its repo is a compile-time error, not a silent fall-through.
            _ => unreachable!("kind validated above"),
        };
        if n == 0 {
            drop(tx);
            return Err(PartyWriteError::PartyNotFound(child_id));
        }
        tx.commit().await?;
        Ok(())
    }
}
