use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use super::AuditMetadata;

/// Strongly-typed ID for PartyContact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PartyContactId(pub Uuid);

impl PartyContactId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PartyContactId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PartyContactId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PartyContactId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PartyContactId> for Uuid {
    fn from(id: PartyContactId) -> Self { id.0 }
}

impl AsRef<Uuid> for PartyContactId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PartyContactId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartyContact {
    pub id: Uuid,
    pub party_id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_primary: bool,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PartyContact {
    /// Create a builder for PartyContact
    pub fn builder() -> PartyContactBuilder {
        PartyContactBuilder::default()
    }

    /// Create a new PartyContact with required fields
    pub fn new(party_id: Uuid, company_id: Uuid, name: String, is_primary: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            party_id,
            company_id,
            name,
            job_title: None,
            department: None,
            email: None,
            phone: None,
            is_primary,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PartyContactId {
        PartyContactId(self.id)
    }

    /// Get when this entity was created
    pub fn created_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.created_at.as_ref()
    }

    /// Get when this entity was last updated
    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.updated_at.as_ref()
    }

    /// Check if this entity is soft deleted
    pub fn is_deleted(&self) -> bool {
        self.metadata.deleted_at.is_some()
    }

    /// Check if this entity is active (not deleted)
    pub fn is_active(&self) -> bool {
        self.metadata.deleted_at.is_none()
    }

    /// Get when this entity was deleted
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.metadata.deleted_at.as_ref()
    }

    /// Get who created this entity
    pub fn created_by(&self) -> Option<&Uuid> {
        self.metadata.created_by.as_ref()
    }

    /// Get who last updated this entity
    pub fn updated_by(&self) -> Option<&Uuid> {
        self.metadata.updated_by.as_ref()
    }

    /// Get who deleted this entity
    pub fn deleted_by(&self) -> Option<&Uuid> {
        self.metadata.deleted_by.as_ref()
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the job_title field (chainable)
    pub fn with_job_title(mut self, value: String) -> Self {
        self.job_title = Some(value);
        self
    }

    /// Set the department field (chainable)
    pub fn with_department(mut self, value: String) -> Self {
        self.department = Some(value);
        self
    }

    /// Set the email field (chainable)
    pub fn with_email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the phone field (chainable)
    pub fn with_phone(mut self, value: String) -> Self {
        self.phone = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "party_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.party_id = v; }
                }
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "job_title" => {
                    if let Ok(v) = serde_json::from_value(value) { self.job_title = v; }
                }
                "department" => {
                    if let Ok(v) = serde_json::from_value(value) { self.department = v; }
                }
                "email" => {
                    if let Ok(v) = serde_json::from_value(value) { self.email = v; }
                }
                "phone" => {
                    if let Ok(v) = serde_json::from_value(value) { self.phone = v; }
                }
                "is_primary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_primary = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PartyContact {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PartyContact"
    }
}

impl backbone_core::PersistentEntity for PartyContact {
    fn entity_id(&self) -> String {
        self.id.to_string()
    }
    fn set_entity_id(&mut self, id: String) {
        if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
            self.id = uuid;
        }
    }
    fn created_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.created_at
    }
    fn set_created_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.created_at = Some(ts);
    }
    fn updated_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.updated_at
    }
    fn set_updated_at(&mut self, ts: chrono::DateTime<chrono::Utc>) {
        self.metadata.updated_at = Some(ts);
    }
    fn deleted_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.metadata.deleted_at
    }
    fn set_deleted_at(&mut self, ts: Option<chrono::DateTime<chrono::Utc>>) {
        self.metadata.deleted_at = ts;
    }
}

impl backbone_orm::EntityRepoMeta for PartyContact {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("party_id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
    fn relations() -> &'static [(&'static str, &'static str, &'static str)] {
        &[("party", "parties", "partyId")]
    }
}

/// Builder for PartyContact entity
///
/// Provides a fluent API for constructing PartyContact instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PartyContactBuilder {
    party_id: Option<Uuid>,
    company_id: Option<Uuid>,
    name: Option<String>,
    job_title: Option<String>,
    department: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    is_primary: Option<bool>,
}

impl PartyContactBuilder {
    /// Set the party_id field (required)
    pub fn party_id(mut self, value: Uuid) -> Self {
        self.party_id = Some(value);
        self
    }

    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the job_title field (optional)
    pub fn job_title(mut self, value: String) -> Self {
        self.job_title = Some(value);
        self
    }

    /// Set the department field (optional)
    pub fn department(mut self, value: String) -> Self {
        self.department = Some(value);
        self
    }

    /// Set the email field (optional)
    pub fn email(mut self, value: String) -> Self {
        self.email = Some(value);
        self
    }

    /// Set the phone field (optional)
    pub fn phone(mut self, value: String) -> Self {
        self.phone = Some(value);
        self
    }

    /// Set the is_primary field (default: `false`)
    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    /// Build the PartyContact entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PartyContact, String> {
        let party_id = self.party_id.ok_or_else(|| "party_id is required".to_string())?;
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let name = self.name.ok_or_else(|| "name is required".to_string())?;

        Ok(PartyContact {
            id: Uuid::new_v4(),
            party_id,
            company_id,
            name,
            job_title: self.job_title,
            department: self.department,
            email: self.email,
            phone: self.phone,
            is_primary: self.is_primary.unwrap_or(false),
            metadata: AuditMetadata::default(),
        })
    }
}
