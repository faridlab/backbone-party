use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "party_status", rename_all = "snake_case")]
pub enum PartyStatus {
    Active,
    Inactive,
    Blocked,
}

impl std::fmt::Display for PartyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Blocked => write!(f, "blocked"),
        }
    }
}

impl FromStr for PartyStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "blocked" => Ok(Self::Blocked),
            _ => Err(format!("Unknown PartyStatus variant: {}", s)),
        }
    }
}

impl Default for PartyStatus {
    fn default() -> Self {
        Self::Active
    }
}
