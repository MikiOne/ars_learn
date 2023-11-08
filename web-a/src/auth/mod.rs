use std::fmt;

use chrono::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use ntex::http::header::AUTHORIZATION;
use ntex::http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;

type AuthResult<T> = Result<T, AppError>;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret miki";

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
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
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenInfo {
    token: String,
}

// pub fn with_auth(role: Role) -> impl Filter + Clone {
//     // 用于检查请求头中的JWT是否有效
//     headers_cloned()
//         .map(move |headers: HeaderMap| (role.clone(), headers))
//         .and_then(authorize)
// }

pub fn create_jwt(uid: String, role: &Role) -> AuthResult<TokenInfo> {
    // 用于创建JWT，参数为用户ID和角色
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamps")
        .timestamp();

    let claims = Claims { sub: uid.to_owned(), role: role.to_string(), exp: expiration as usize };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
        .map_err(|_| AppError::JWTTokenCreationError)?;
    Ok(TokenInfo { token })
}

fn authorize((role, headers): (Role, HeaderMap)) -> AuthResult<String> {
    // 用于验证JWT的有效性和角色权限
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            info!("jwt: {:?}", jwt);
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
                .map_err(|_| AppError::JWTTokenError)?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(AppError::NoPermissionError);
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(e),
    }
}

fn jwt_from_header(headers: &HeaderMap) -> AuthResult<String> {
    // 用于从请求头中获取JWT
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AppError::NoAuthHeaderError),
    };

    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(AppError::InvalidAuthHeaderError),
    };

    if !auth_header.starts_with(BEARER) {
        return Err(AppError::InvalidAuthHeaderError);
    }

    info!("auth_header: {}", auth_header);
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
