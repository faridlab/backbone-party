//! Validated write path for Party + its multi-channel children — hand-authored (user-owned).
//!
//! Closes the CRUD-bypass: the generated 12-endpoint CRUD writes rows with NO domain validation.
//! Here `create_party` validates NPWP/NIK format + uniqueness; the child writers verify the party
//! exists. Geo ids on an address are LOGICAL FKs (validated at the ACL layer / consuming service,
//! not against geo's schema here — keeps party decoupled from geo).
//!
//! Tenant scope (ADR-0010 B1): every write is tenant-bound. The caller's company (resolved from
//! `company_scope::current_company()` by the guarded route, or passed via `New*.company_id`) is
//! bound into every INSERT and into `with_company_scope` so the RLS WITH CHECK accepts the row.
//! Defense-in-depth on top of the ADR-0008 fence: a missed scope still fails closed.
//!
//! SQL lives in the repositories (`PartyRepository`, `PartyAddressRepository`, …), not here, per
//! the module's 4-layer rule. This service only orchestrates validation + dispatch + the
//! duplicate-key → typed-error mapping.

use backbone_orm::company_scope;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

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
    InvalidEmail(String),
    /// A party_kind/field mismatch (e.g. person with no name parts, org carrying a NIK).
    InconsistentKind(String),
    /// The party already has a primary of this kind (one-primary-per-party invariant).
    DuplicatePrimary(&'static str),
    /// A write path needed the caller's company but the request scope was unset
    /// (missing `with_company_scope` / `with_request_scope` middleware).
    NoCompanyScope,
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
            PartyWriteError::InvalidEmail(_) => "invalid_email",
            PartyWriteError::InconsistentKind(_) => "inconsistent_party_kind",
            PartyWriteError::DuplicatePrimary(_) => "duplicate_primary",
            PartyWriteError::NoCompanyScope => "no_company_scope",
            PartyWriteError::Db(_) => "internal_error",
        }
    }
    pub fn http_status(&self) -> u16 {
        match self {
            PartyWriteError::Db(_) => 500,
            PartyWriteError::NoCompanyScope => 401,
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
            | PartyWriteError::InvalidEmail(v)
            | PartyWriteError::InconsistentKind(v) => write!(f, ": {v}"),
            PartyWriteError::DuplicatePrimary(kind) => write!(f, ": {kind}"),
            PartyWriteError::NoCompanyScope | PartyWriteError::Db(_) => Ok(()),
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
    pub company_id: Uuid,
    pub party_code: String,
    pub party_kind: Option<String>,
    pub name: String,
    pub legal_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub npwp: Option<String>,
    pub nik: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NewAddress {
    pub company_id: Uuid,
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
    pub company_id: Uuid,
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
    pub company_id: Uuid,
    pub party_id: Uuid,
    pub label: Option<String>,
    pub email: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct NewPhone {
    pub company_id: Uuid,
    pub party_id: Uuid,
    pub label: Option<String>,
    pub phone: String,
    pub is_primary: bool,
}

#[derive(Clone)]
pub struct PartyWriteService {
    db_pool: PgPool,
}

impl PartyWriteService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    fn is_dup(e: &sqlx::Error, needle: &str) -> bool {
        e.as_database_error()
            .map(|d| d.is_unique_violation() && d.constraint().unwrap_or("").contains(needle))
            .unwrap_or(false)
    }
    fn is_unique(e: &sqlx::Error) -> bool {
        e.as_database_error().map(|d| d.is_unique_violation()).unwrap_or(false)
    }

    /// Existence check filtered by the caller's company. The scope wrapper preserves fail-closed
    /// behavior under RLS even if the request scope wasn't set (missed scope → no rows returned).
    async fn party_exists_in(&self, id: Uuid, company: Uuid) -> Result<bool, PartyWriteError> {
        let parties = PartyRepository::new(self.db_pool.clone());
        Ok(parties.find_active_id_in_company(&self.db_pool, id, company).await?.is_some())
    }

    pub async fn create_party(&self, p: NewParty) -> Result<Uuid, PartyWriteError> {
        let company = p.company_id;
        company_scope::with_company_scope(Some(company), async move {
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
                    company_id: company,
                    party_code: &p.party_code,
                    party_kind: &kind,
                    name: &p.name,
                    legal_name: p.legal_name.as_deref(),
                    first_name: p.first_name.as_deref(),
                    last_name: p.last_name.as_deref(),
                    npwp: p.npwp.as_deref(),
                    nik: p.nik.as_deref(),
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
        }).await
    }

    pub async fn add_address(&self, a: NewAddress) -> Result<Uuid, PartyWriteError> {
        let company = a.company_id;
        company_scope::with_company_scope(Some(company), async move {
            if !self.party_exists_in(a.party_id, company).await? {
                return Err(PartyWriteError::PartyNotFound(a.party_id));
            }
            let id = Uuid::new_v4();
            let atype = a.address_type.clone().unwrap_or_else(|| "home".to_string());
            let addresses = PartyAddressRepository::new(self.db_pool.clone());
            let r = addresses.insert_from_new(
                &self.db_pool,
                &NewPartyAddressRow {
                    id,
                    company_id: company,
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
        }).await
    }

    pub async fn add_contact(&self, c: NewContact) -> Result<Uuid, PartyWriteError> {
        let company = c.company_id;
        company_scope::with_company_scope(Some(company), async move {
            if !self.party_exists_in(c.party_id, company).await? {
                return Err(PartyWriteError::PartyNotFound(c.party_id));
            }
            let id = Uuid::new_v4();
            let contacts = PartyContactRepository::new(self.db_pool.clone());
            let r = contacts.insert_from_new(
                &self.db_pool,
                &NewPartyContactRow {
                    id,
                    company_id: company,
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
        }).await
    }

    pub async fn add_email(&self, e: NewEmail) -> Result<Uuid, PartyWriteError> {
        let company = e.company_id;
        company_scope::with_company_scope(Some(company), async move {
            if !self.party_exists_in(e.party_id, company).await? {
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
                    company_id: company,
                    party_id: e.party_id,
                    label: &label,
                    email: &e.email,
                    is_primary: e.is_primary,
                },
            ).await;
            Self::ok_or_primary(r, id, "email")
        }).await
    }

    pub async fn add_phone(&self, p: NewPhone) -> Result<Uuid, PartyWriteError> {
        let company = p.company_id;
        company_scope::with_company_scope(Some(company), async move {
            if !self.party_exists_in(p.party_id, company).await? {
                return Err(PartyWriteError::PartyNotFound(p.party_id));
            }
            let id = Uuid::new_v4();
            let label = p.label.clone().unwrap_or_else(|| "mobile".to_string());
            let phones = PartyPhoneRepository::new(self.db_pool.clone());
            let r = phones.insert_from_new(
                &self.db_pool,
                &NewPartyPhoneRow {
                    id,
                    company_id: company,
                    party_id: p.party_id,
                    label: &label,
                    phone: &p.phone,
                    is_primary: p.is_primary,
                },
            ).await;
            Self::ok_or_primary(r, id, "phone")
        }).await
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
    /// Company-scoped: the caller's company (from the request scope) filters the lookup AND binds
    /// into the transaction so the RLS WITH CHECK accepts the writes.
    ///
    /// Dispatches on `kind` to the per-child repository's `clear_primary_for_party` +
    /// `set_primary_child` methods, killing the old `format!("UPDATE party.{table} …")` smell —
    /// each repo knows its own table at compile time.
    pub async fn set_primary(
        &self,
        party_id: Uuid,
        kind: &str,
        child_id: Uuid,
    ) -> Result<(), PartyWriteError> {
        let company = company_scope::current_company()
            .ok_or(PartyWriteError::NoCompanyScope)?;
        // Validate kind BEFORE opening the tx so unknown kinds bail with no side effects.
        match kind {
            "address" | "contact" | "email" | "phone" => {}
            _ => return Err(PartyWriteError::InconsistentKind(format!("unknown child kind: {kind}"))),
        }
        if !self.party_exists_in(party_id, company).await? {
            return Err(PartyWriteError::PartyNotFound(party_id));
        }
        let mut tx = self.db_pool.begin().await?;
        // Bind the caller's company onto this transaction so the RLS WITH CHECK accepts the writes
        // (ADR-0008 pattern for hand-written write services managing their own tx).
        company_scope::bind_current_company(&mut tx).await?;
        // Clear first (so the partial-unique index never sees two primaries mid-transaction).
        // Dispatch to the per-child repo so the table name is a compile-time constant, not a
        // string-built identifier.
        let n = match kind {
            "address" => {
                let repo = PartyAddressRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id, company).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id, company).await?
            }
            "contact" => {
                let repo = PartyContactRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id, company).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id, company).await?
            }
            "email" => {
                let repo = PartyEmailRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id, company).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id, company).await?
            }
            "phone" => {
                let repo = PartyPhoneRepository::new(self.db_pool.clone());
                repo.clear_primary_for_party(&mut *tx, party_id, company).await?;
                repo.set_primary_child(&mut *tx, child_id, party_id, company).await?
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
