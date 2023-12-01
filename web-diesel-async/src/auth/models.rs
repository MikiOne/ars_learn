use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SlimUser {
    pub uid: String,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

impl Role {
    pub fn from_str(role: &str) -> Self {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenInfo {
    pub token: String,
}
