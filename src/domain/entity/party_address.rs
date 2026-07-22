use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::AddressType;
use super::PartyStatus;
use super::AuditMetadata;

/// Strongly-typed ID for PartyAddress
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PartyAddressId(pub Uuid);

impl PartyAddressId {
    pub fn new(id: Uuid) -> Self { Self(id) }
    pub fn generate() -> Self { Self(Uuid::new_v4()) }
    pub fn into_inner(self) -> Uuid { self.0 }
}

impl std::fmt::Display for PartyAddressId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for PartyAddressId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl From<Uuid> for PartyAddressId {
    fn from(id: Uuid) -> Self { Self(id) }
}

impl From<PartyAddressId> for Uuid {
    fn from(id: PartyAddressId) -> Self { id.0 }
}

impl AsRef<Uuid> for PartyAddressId {
    fn as_ref(&self) -> &Uuid { &self.0 }
}

impl std::ops::Deref for PartyAddressId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartyAddress {
    pub id: Uuid,
    pub party_id: Uuid,
    pub company_id: Uuid,
    pub address_type: AddressType,
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
    pub status: PartyStatus,
    #[serde(default)]
    #[sqlx(json)]
    pub metadata: AuditMetadata,
}

impl PartyAddress {
    /// Create a builder for PartyAddress
    pub fn builder() -> PartyAddressBuilder {
        PartyAddressBuilder::default()
    }

    /// Create a new PartyAddress with required fields
    pub fn new(party_id: Uuid, company_id: Uuid, address_type: AddressType, line1: String, is_primary: bool, is_billing: bool, is_shipping: bool, status: PartyStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            party_id,
            company_id,
            address_type,
            label: None,
            line1,
            line2: None,
            country_id: None,
            province_id: None,
            city_id: None,
            district_id: None,
            subdistrict_id: None,
            postal_code: None,
            latitude: None,
            longitude: None,
            is_primary,
            is_billing,
            is_shipping,
            status,
            metadata: AuditMetadata::default(),
        }
    }

    /// Get the entity's unique identifier
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get a strongly-typed ID for this entity
    pub fn typed_id(&self) -> PartyAddressId {
        PartyAddressId(self.id)
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

    /// Set the label field (chainable)
    pub fn with_label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    /// Set the line2 field (chainable)
    pub fn with_line2(mut self, value: String) -> Self {
        self.line2 = Some(value);
        self
    }

    /// Set the country_id field (chainable)
    pub fn with_country_id(mut self, value: Uuid) -> Self {
        self.country_id = Some(value);
        self
    }

    /// Set the province_id field (chainable)
    pub fn with_province_id(mut self, value: Uuid) -> Self {
        self.province_id = Some(value);
        self
    }

    /// Set the city_id field (chainable)
    pub fn with_city_id(mut self, value: Uuid) -> Self {
        self.city_id = Some(value);
        self
    }

    /// Set the district_id field (chainable)
    pub fn with_district_id(mut self, value: Uuid) -> Self {
        self.district_id = Some(value);
        self
    }

    /// Set the subdistrict_id field (chainable)
    pub fn with_subdistrict_id(mut self, value: Uuid) -> Self {
        self.subdistrict_id = Some(value);
        self
    }

    /// Set the postal_code field (chainable)
    pub fn with_postal_code(mut self, value: String) -> Self {
        self.postal_code = Some(value);
        self
    }

    /// Set the latitude field (chainable)
    pub fn with_latitude(mut self, value: Decimal) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (chainable)
    pub fn with_longitude(mut self, value: Decimal) -> Self {
        self.longitude = Some(value);
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
                "address_type" => {
                    if let Ok(v) = serde_json::from_value(value) { self.address_type = v; }
                }
                "label" => {
                    if let Ok(v) = serde_json::from_value(value) { self.label = v; }
                }
                "line1" => {
                    if let Ok(v) = serde_json::from_value(value) { self.line1 = v; }
                }
                "line2" => {
                    if let Ok(v) = serde_json::from_value(value) { self.line2 = v; }
                }
                "country_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.country_id = v; }
                }
                "province_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.province_id = v; }
                }
                "city_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.city_id = v; }
                }
                "district_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.district_id = v; }
                }
                "subdistrict_id" => {
                    if let Ok(v) = serde_json::from_value(value) { self.subdistrict_id = v; }
                }
                "postal_code" => {
                    if let Ok(v) = serde_json::from_value(value) { self.postal_code = v; }
                }
                "latitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.latitude = v; }
                }
                "longitude" => {
                    if let Ok(v) = serde_json::from_value(value) { self.longitude = v; }
                }
                "is_primary" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_primary = v; }
                }
                "is_billing" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_billing = v; }
                }
                "is_shipping" => {
                    if let Ok(v) = serde_json::from_value(value) { self.is_shipping = v; }
                }
                "status" => {
                    if let Ok(v) = serde_json::from_value(value) { self.status = v; }
                }
                _ => {} // ignore unknown fields
            }
        }
    }

    // <<< CUSTOM METHODS START >>>
    // <<< CUSTOM METHODS END >>>
}

impl super::Entity for PartyAddress {
    type Id = Uuid;

    fn entity_id(&self) -> &Self::Id {
        &self.id
    }

    fn entity_type() -> &'static str {
        "PartyAddress"
    }
}

impl backbone_core::PersistentEntity for PartyAddress {
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

impl backbone_orm::EntityRepoMeta for PartyAddress {
    fn column_types() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("id".to_string(), "uuid".to_string());
        m.insert("party_id".to_string(), "uuid".to_string());
        m.insert("company_id".to_string(), "uuid".to_string());
        m.insert("country_id".to_string(), "uuid".to_string());
        m.insert("province_id".to_string(), "uuid".to_string());
        m.insert("city_id".to_string(), "uuid".to_string());
        m.insert("district_id".to_string(), "uuid".to_string());
        m.insert("subdistrict_id".to_string(), "uuid".to_string());
        m.insert("address_type".to_string(), "address_type".to_string());
        m.insert("status".to_string(), "party_status".to_string());
        m
    }
    fn search_fields() -> &'static [&'static str] {
        &["line1"]
    }
    fn company_field() -> Option<&'static str> {
        Some("company_id")
    }
    fn relations() -> &'static [(&'static str, &'static str, &'static str)] {
        &[("party", "parties", "partyId")]
    }
}

/// Builder for PartyAddress entity
///
/// Provides a fluent API for constructing PartyAddress instances.
/// System fields (id, metadata, timestamps) are auto-initialized.
#[derive(Debug, Clone, Default)]
pub struct PartyAddressBuilder {
    party_id: Option<Uuid>,
    company_id: Option<Uuid>,
    address_type: Option<AddressType>,
    label: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    country_id: Option<Uuid>,
    province_id: Option<Uuid>,
    city_id: Option<Uuid>,
    district_id: Option<Uuid>,
    subdistrict_id: Option<Uuid>,
    postal_code: Option<String>,
    latitude: Option<Decimal>,
    longitude: Option<Decimal>,
    is_primary: Option<bool>,
    is_billing: Option<bool>,
    is_shipping: Option<bool>,
    status: Option<PartyStatus>,
}

impl PartyAddressBuilder {
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

    /// Set the address_type field (default: `AddressType::default()`)
    pub fn address_type(mut self, value: AddressType) -> Self {
        self.address_type = Some(value);
        self
    }

    /// Set the label field (optional)
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }

    /// Set the line1 field (required)
    pub fn line1(mut self, value: String) -> Self {
        self.line1 = Some(value);
        self
    }

    /// Set the line2 field (optional)
    pub fn line2(mut self, value: String) -> Self {
        self.line2 = Some(value);
        self
    }

    /// Set the country_id field (optional)
    pub fn country_id(mut self, value: Uuid) -> Self {
        self.country_id = Some(value);
        self
    }

    /// Set the province_id field (optional)
    pub fn province_id(mut self, value: Uuid) -> Self {
        self.province_id = Some(value);
        self
    }

    /// Set the city_id field (optional)
    pub fn city_id(mut self, value: Uuid) -> Self {
        self.city_id = Some(value);
        self
    }

    /// Set the district_id field (optional)
    pub fn district_id(mut self, value: Uuid) -> Self {
        self.district_id = Some(value);
        self
    }

    /// Set the subdistrict_id field (optional)
    pub fn subdistrict_id(mut self, value: Uuid) -> Self {
        self.subdistrict_id = Some(value);
        self
    }

    /// Set the postal_code field (optional)
    pub fn postal_code(mut self, value: String) -> Self {
        self.postal_code = Some(value);
        self
    }

    /// Set the latitude field (optional)
    pub fn latitude(mut self, value: Decimal) -> Self {
        self.latitude = Some(value);
        self
    }

    /// Set the longitude field (optional)
    pub fn longitude(mut self, value: Decimal) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Set the is_primary field (default: `false`)
    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    /// Set the is_billing field (default: `false`)
    pub fn is_billing(mut self, value: bool) -> Self {
        self.is_billing = Some(value);
        self
    }

    /// Set the is_shipping field (default: `false`)
    pub fn is_shipping(mut self, value: bool) -> Self {
        self.is_shipping = Some(value);
        self
    }

    /// Set the status field (default: `PartyStatus::default()`)
    pub fn status(mut self, value: PartyStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Build the PartyAddress entity
    ///
    /// Returns Err if any required field without a default is missing.
    pub fn build(self) -> Result<PartyAddress, String> {
        let party_id = self.party_id.ok_or_else(|| "party_id is required".to_string())?;
        let company_id = self.company_id.ok_or_else(|| "company_id is required".to_string())?;
        let line1 = self.line1.ok_or_else(|| "line1 is required".to_string())?;

        Ok(PartyAddress {
            id: Uuid::new_v4(),
            party_id,
            company_id,
            address_type: self.address_type.unwrap_or(AddressType::default()),
            label: self.label,
            line1,
            line2: self.line2,
            country_id: self.country_id,
            province_id: self.province_id,
            city_id: self.city_id,
            district_id: self.district_id,
            subdistrict_id: self.subdistrict_id,
            postal_code: self.postal_code,
            latitude: self.latitude,
            longitude: self.longitude,
            is_primary: self.is_primary.unwrap_or(false),
            is_billing: self.is_billing.unwrap_or(false),
            is_shipping: self.is_shipping.unwrap_or(false),
            status: self.status.unwrap_or(PartyStatus::default()),
            metadata: AuditMetadata::default(),
        })
    }
}
