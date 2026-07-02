use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "address_type", rename_all = "snake_case")]
pub enum AddressType {
    Billing,
    Shipping,
    Office,
    Home,
    Warehouse,
    Other,
}

impl std::fmt::Display for AddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Billing => write!(f, "billing"),
            Self::Shipping => write!(f, "shipping"),
            Self::Office => write!(f, "office"),
            Self::Home => write!(f, "home"),
            Self::Warehouse => write!(f, "warehouse"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl FromStr for AddressType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "billing" => Ok(Self::Billing),
            "shipping" => Ok(Self::Shipping),
            "office" => Ok(Self::Office),
            "home" => Ok(Self::Home),
            "warehouse" => Ok(Self::Warehouse),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown AddressType variant: {}", s)),
        }
    }
}

impl Default for AddressType {
    fn default() -> Self {
        Self::Home
    }
}
