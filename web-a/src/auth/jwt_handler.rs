use crate::auth::error::AuthError;
use chrono::prelude::*;
use jsonwebtoken::decode;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use ntex::http::header::AUTHORIZATION;
use ntex::http::HeaderMap;

use crate::auth::models::{BaseUser, Claims, Role, TokenInfo};

type AuthResult<T> = Result<T, AuthError>;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret mi";

pub fn create_jwt(uid: String, role: &Role) -> AuthResult<TokenInfo> {
    // 用于创建JWT，参数为用户ID和角色
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(120))
        .expect("valid timestamps")
        .timestamp();

    let claims = Claims { sub: uid.to_owned(), role: role.to_string(), exp: expiration as usize };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| AuthError::JWTTokenCreationError)?;
    Ok(TokenInfo { token })
}

pub fn get_jwt_user(headers: &HeaderMap) -> AuthResult<BaseUser> {
    let jwt = jwt_from_header(headers)?;
    decode_jwt(jwt)
}

pub fn decode_jwt(jwt: String) -> AuthResult<BaseUser> {
    let dkey = DecodingKey::from_secret(JWT_SECRET);
    let valid = Validation::new(Algorithm::HS512);

    info!("jwt: {:?}", jwt);
    let decoded = decode::<Claims>(&jwt, &dkey, &valid).map_err(|err| {
        error!("decode jwt error: {:?}", err);
        AuthError::JWTTokenError
    })?;

    Ok(BaseUser { uid: decoded.claims.sub })
}

pub fn jwt_from_header(headers: &HeaderMap) -> AuthResult<String> {
    // 用于从请求头中获取JWT
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(AuthError::NoAuthHeaderError),
    };

    let auth_header = header.to_str().map_err(|err| {
        error!("parse jwt from header. toStr error: {:?}", err);
        return AuthError::InvalidAuthHeaderError;
    })?;

    if !auth_header.starts_with(BEARER) {
        return Err(AuthError::InvalidAuthHeaderError);
    }

    info!("auth_header: {}", auth_header);
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
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
            .map_err(|_| AuthError::JWTTokenError)?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(AuthError::NoPermissionError);
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(e),
    }
}
