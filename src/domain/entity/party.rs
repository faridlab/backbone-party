use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::PartyKind;
use super::PartyStatus;
use super::AuditMetadata;

/// Strongly-typed ID for Party
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PartyId(pub Uuid);

impl PartyId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PartyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PartyId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PartyId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PartyId> for Uuid {
    fn from(id: PartyId) -> Self { id.0 }
}

impl AsRef<Uuid> for PartyId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PartyId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: Uuid,
    pub company_id: Uuid,
    pub party_code: String,
    pub party_kind: PartyKind,
    pub name: String,
    pub legal_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub npwp: Option<String>,
    pub nik: Option<String>,
    pub status: PartyStatus,
    pub notes: Option<String>,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl Party {
    /// Create a builder for Party
    pub fn builder() -> PartyBuilder {
        PartyBuilder::default()
    }

    /// Create a new Party with required fields
    pub fn new(company_id: Uuid, party_code: String, party_kind: PartyKind, name: String, status: PartyStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id,
            party_code,
            party_kind,
            name,
            legal_name: None,
            first_name: None,
            last_name: None,
            npwp: None,
            nik: None,
            status,
            notes: None,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PartyId {
        PartyId(self.id)
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

    /// Get the current status
    pub fn status(&self) -> &PartyStatus {
        &self.status
    }


    // ==========================================================
    // Fluent Setters (with_* for optional fields)
    // ==========================================================

    /// Set the legal_name field (chainable)
    pub fn with_legal_name(mut self, value: String) -> Self {
        self.legal_name = Some(value);
        self
    }

    /// Set the first_name field (chainable)
    pub fn with_first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    /// Set the last_name field (chainable)
    pub fn with_last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the npwp field (chainable)
    pub fn with_npwp(mut self, value: String) -> Self {
        self.npwp = Some(value);
        self
    }

    /// Set the nik field (chainable)
    pub fn with_nik(mut self, value: String) -> Self {
        self.nik = Some(value);
        self
    }

    /// Set the notes field (chainable)
    pub fn with_notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }

    // ==========================================================
    // Partial Update
    // ==========================================================

    /// Apply partial updates from a map of field name to JSON value
    pub fn apply_patch(&mut self, fields: std::collections::HashMap<String, serde_json::Value>) {
        for (key, value) in fields {
            match key.as_str() {
                "company_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.company_id = v; }
                }
                "party_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.party_code = v; }
                }
                "party_kind" => {
                    if let Ok(v) = serde_json::from_value(value) { self.party_kind = v; }
                }
                "name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.name = v; }
                }
                "legal_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.legal_name = v; }
                }
                "first_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.first_name = v; }
                }
                "last_name" => {
                    if let Ok(v) = serde_json::from_value(value) { self.last_name = v; }
                }
                "npwp" => {
                    if let Ok(v) = serde_json::from_value(value) { self.npwp = v; }
                }
                "nik" => {
                    if let Ok(v) = serde_json::from_value(value) { self.nik = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                "notes" => {
                    if let Ok(v) = serde_json::from_value(value) { self.notes = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for Party {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "Party"
    }
}

impl backbone_core::PersistentEntity for Party {
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

impl backbone_orm::EntityRepoMeta for Party {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("party_kind".to_string(), "party_kind".to_string());
        m.insert("status".to_string(), "party_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["party_code", "name"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
}

/// Builder for Party entity
///
/// Provides a fluent API for constructing Party instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PartyBuilder {
    company_id: Option<Uuid>,
    party_code: Option<String>,
    party_kind: Option<PartyKind>,
    name: Option<String>,
    legal_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    npwp: Option<String>,
    nik: Option<String>,
    status: Option<PartyStatus>,
    notes: Option<String>,
}

impl PartyBuilder {
    /// Set the company_id field (required)
    pub fn company_id(mut self, value: Uuid) -> Self {
        self.company_id = Some(value);
        self
    }

    /// Set the party_code field (required)
    pub fn party_code(mut self, value: String) -> Self {
        self.party_code = Some(value);
        self
    }

    /// Set the party_kind field (default: `PartyKind::default()`)
    pub fn party_kind(mut self, value: PartyKind) -> Self {
        self.party_kind = Some(value);
        self
    }

    /// Set the name field (required)
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the legal_name field (optional)
    pub fn legal_name(mut self, value: String) -> Self {
        self.legal_name = Some(value);
        self
    }

    /// Set the first_name field (optional)
    pub fn first_name(mut self, value: String) -> Self {
        self.first_name = Some(value);
        self
    }

    /// Set the last_name field (optional)
    pub fn last_name(mut self, value: String) -> Self {
        self.last_name = Some(value);
        self
    }

    /// Set the npwp field (optional)
    pub fn npwp(mut self, value: String) -> Self {
        self.npwp = Some(value);
        self
    }

    /// Set the nik field (optional)
    pub fn nik(mut self, value: String) -> Self {
        self.nik = Some(value);
        self
    }

    /// Set the status field (default: `PartyStatus::default()`)
    pub fn status(mut self, value: PartyStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Set the notes field (optional)
    pub fn notes(mut self, value: String) -> Self {
        self.notes = Some(value);
        self
    }

    /// Build the Party entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<Party, String> {
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let party_code = self.party_code.ok_or_else(|| "party_code is required".to_string())?;
        let name = self.name.ok_or_else(|| "name is required".to_string())?;

        Ok(Party {
            id: Uuid::new_v4(),
            company_id,
            party_code,
            party_kind: self.party_kind.unwrap_or(PartyKind::default()),
            name,
            legal_name: self.legal_name,
            first_name: self.first_name,
            last_name: self.last_name,
            npwp: self.npwp,
            nik: self.nik,
            status: self.status.unwrap_or(PartyStatus::default()),
            notes: self.notes,
            metadata: AuditMetadata::default(),
        })
    }
}
