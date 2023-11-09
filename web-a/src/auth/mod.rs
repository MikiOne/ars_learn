use std::fmt;

use chrono::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::decode;
use log::{error, info};
use ntex::http::header::AUTHORIZATION;
use ntex::http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;

pub mod filter;

type AuthResult<T> = Result<T, AppError>;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret miki";

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseUser {
    uid: String,
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

pub fn create_jwt(uid: String, role: &Role) -> AuthResult<TokenInfo> {
    // 用于创建JWT，参数为用户ID和角色
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
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

pub fn authorize((role, headers): (Role, HeaderMap)) -> AuthResult<String> {
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

pub fn user_from_header(headers: HeaderMap) -> AuthResult<BaseUser> {
    let jwt = jwt_from_header(&headers)?;
    decode_jwt(jwt)
}

pub fn decode_jwt(jwt: String) -> AuthResult<BaseUser> {
    let dkey = DecodingKey::from_secret(JWT_SECRET);
    let valid = Validation::new(Algorithm::HS512);

    info!("jwt: {:?}", jwt);
    let decoded = decode::<Claims>(&jwt, &dkey, &valid).map_err(|err| {
        error!("decode jwt error: {:?}", err);
        AppError::JWTTokenError
    })?;

    Ok(BaseUser { uid: decoded.claims.sub })
}

pub fn jwt_from_header(headers: &HeaderMap) -> AuthResult<String> {
    // 用于从请求头中获取JWT
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AppError::NoAuthHeaderError),
    };

    let auth_header = header.to_str().map_err(|err| {
        error!("parse jwt from header. toStr error: {:?}", err);
        return AppError::InvalidAuthHeaderError;
    })?;

    if !auth_header.starts_with(BEARER) {
        return Err(AppError::InvalidAuthHeaderError);
    }

    info!("auth_header: {}", auth_header);
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
