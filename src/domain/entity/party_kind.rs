use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "party_kind", rename_all = "snake_case")]
pub enum PartyKind {
    Organization,
    Person,
}

impl std::fmt::Display for PartyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Organization => write!(f, "organization"),
            Self::Person => write!(f, "person"),
        }
    }
}

impl FromStr for PartyKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "organization" => Ok(Self::Organization),
            "person" => Ok(Self::Person),
            _ => Err(format!("Unknown PartyKind variant: {}", s)),
        }
    }
}

impl Default for PartyKind {
    fn default() -> Self {
        Self::Organization
    }
}
