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

use backbone_orm::company_scope;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

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

    /// Existence check filtered by the caller's company. The `($2::uuid IS NULL OR company_id = $2)`
    /// shape preserves fail-closed behavior under RLS even if the request scope wasn't set
    /// (missed scope → no rows returned).
    async fn party_exists_in(&self, id: Uuid, company: Uuid) -> Result<bool, PartyWriteError> {
        let found: Option<Uuid> = sqlx::query_scalar(
            "SELECT id FROM party.parties \
             WHERE id = $1 AND company_id = $2 AND (metadata->>'deleted_at') IS NULL",
        )
        .bind(id)
        .bind(company)
        .fetch_optional(&self.db_pool)
        .await?;
        Ok(found.is_some())
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
            let r = sqlx::query(
                r#"INSERT INTO party.parties
                    (id, company_id, party_code, party_kind, name, legal_name, first_name, last_name,
                     npwp, nik, status)
                   VALUES ($1,$2,$3,$4::party_kind,$5,$6,$7,$8,$9,$10,'active'::party_status)"#,
            )
            .bind(id)
            .bind(company)
            .bind(&p.party_code)
            .bind(&kind)
            .bind(&p.name)
            .bind(&p.legal_name)
            .bind(&p.first_name)
            .bind(&p.last_name)
            .bind(&p.npwp)
            .bind(&p.nik)
            .execute(&self.db_pool)
            .await;
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
            let r = sqlx::query(
                r#"INSERT INTO party.party_addresses
                    (id, company_id, party_id, address_type, label, line1, line2, country_id, province_id,
                     city_id, district_id, subdistrict_id, postal_code, latitude, longitude, is_primary,
                     is_billing, is_shipping, status)
                   VALUES ($1,$2,$3,$4::address_type,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,'active'::party_status)"#,
            )
            .bind(id)
            .bind(company)
            .bind(a.party_id)
            .bind(&atype)
            .bind(&a.label)
            .bind(&a.line1)
            .bind(&a.line2)
            .bind(a.country_id)
            .bind(a.province_id)
            .bind(a.city_id)
            .bind(a.district_id)
            .bind(a.subdistrict_id)
            .bind(&a.postal_code)
            .bind(a.latitude)
            .bind(a.longitude)
            .bind(a.is_primary)
            .bind(a.is_billing)
            .bind(a.is_shipping)
            .execute(&self.db_pool)
            .await;
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
            let r = sqlx::query(
                r#"INSERT INTO party.party_contacts
                    (id, company_id, party_id, name, job_title, department, email, phone, is_primary)
                   VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#,
            )
            .bind(id)
            .bind(company)
            .bind(c.party_id)
            .bind(&c.name)
            .bind(&c.job_title)
            .bind(&c.department)
            .bind(&c.email)
            .bind(&c.phone)
            .bind(c.is_primary)
            .execute(&self.db_pool)
            .await;
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
            let r = sqlx::query(
                "INSERT INTO party.party_emails (id, company_id, party_id, label, email, is_primary) \
                 VALUES ($1,$2,$3,$4,$5,$6)",
            )
            .bind(id)
            .bind(company)
            .bind(e.party_id)
            .bind(&label)
            .bind(&e.email)
            .bind(e.is_primary)
            .execute(&self.db_pool)
            .await;
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
            let r = sqlx::query(
                "INSERT INTO party.party_phones (id, company_id, party_id, label, phone, is_primary) \
                 VALUES ($1,$2,$3,$4,$5,$6)",
            )
            .bind(id)
            .bind(company)
            .bind(p.party_id)
            .bind(&label)
            .bind(&p.phone)
            .bind(p.is_primary)
            .execute(&self.db_pool)
            .await;
            Self::ok_or_primary(r, id, "phone")
        }).await
    }

    fn ok_or_primary(
        r: Result<sqlx::postgres::PgQueryResult, sqlx::Error>,
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
    pub async fn set_primary(
        &self,
        party_id: Uuid,
        kind: &str,
        child_id: Uuid,
    ) -> Result<(), PartyWriteError> {
        let company = company_scope::current_company()
            .ok_or(PartyWriteError::NoCompanyScope)?;
        let table = match kind {
            "address" => "party_addresses",
            "contact" => "party_contacts",
            "email" => "party_emails",
            "phone" => "party_phones",
            _ => return Err(PartyWriteError::InconsistentKind(format!("unknown child kind: {kind}"))),
        };
        if !self.party_exists_in(party_id, company).await? {
            return Err(PartyWriteError::PartyNotFound(party_id));
        }
        let mut tx = self.db_pool.begin().await?;
        // Bind the caller's company onto this transaction so the RLS WITH CHECK accepts the writes
        // (ADR-0008 pattern for hand-written write services managing their own tx).
        company_scope::bind_current_company(&mut tx).await?;
        // Clear first (so the partial-unique index never sees two primaries mid-transaction).
        sqlx::query(&format!(
            "UPDATE party.{table} SET is_primary = FALSE WHERE party_id = $1 AND company_id = $2"
        ))
        .bind(party_id).bind(company).execute(&mut *tx).await?;
        let n = sqlx::query(&format!(
            "UPDATE party.{table} SET is_primary = TRUE \
             WHERE id = $1 AND party_id = $2 AND company_id = $3 AND (metadata->>'deleted_at') IS NULL"
        ))
        .bind(child_id).bind(party_id).bind(company).execute(&mut *tx).await?;
        if n.rows_affected() == 0 {
            drop(tx);
            return Err(PartyWriteError::PartyNotFound(child_id));
        }
        tx.commit().await?;
        Ok(())
    }
}
