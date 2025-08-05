use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    PropertyManager,
    Tenant,
    Landlord,
    Maintenance,
    Guest,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "super_admin"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::PropertyManager => write!(f, "property_manager"),
            UserRole::Tenant => write!(f, "tenant"),
            UserRole::Landlord => write!(f, "landlord"),
            UserRole::Maintenance => write!(f, "maintenance"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "super_admin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "property_manager" => Ok(UserRole::PropertyManager),
            "tenant" => Ok(UserRole::Tenant),
            "landlord" => Ok(UserRole::Landlord),
            "maintenance" => Ok(UserRole::Maintenance),
            "guest" => Ok(UserRole::Guest),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdentifierType {
    Email,
    Phone,
}