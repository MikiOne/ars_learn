use chrono::prelude::*;
use jsonwebtoken::decode;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use ntex::http::header::AUTHORIZATION;
use ntex::http::HeaderMap;

use crate::auth::models::{Claims, Role, SlimUser, TokenInfo};
use crate::common::biz_code::BizCode;
use crate::common::biz_error::BizError;
use crate::common::biz_resp::BizResult;

const BEARER: &str = "Bearer ";
const JWT_SECRET: &[u8] = b"secret mi";

pub fn create_jwt(uid: String, role: &Role) -> BizResult<TokenInfo> {
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
    .map_err(|err| {
        error!("generate JWT error: {:?}", err);
        BizError::new(BizCode::JWT_CREATION_ERR)
    })?;
    Ok(TokenInfo { token })
}

pub fn get_jwt_user(headers: &HeaderMap) -> BizResult<SlimUser> {
    let jwt = jwt_from_header(headers)?;
    decode_jwt(jwt)
}

pub fn decode_jwt(jwt: String) -> BizResult<SlimUser> {
    let dkey = DecodingKey::from_secret(JWT_SECRET);
    let valid = Validation::new(Algorithm::HS512);

    info!("input jwt: {:?}", jwt);
    let decoded = decode::<Claims>(&jwt, &dkey, &valid).map_err(|err| {
        error!("decode jwt error: {:?}", err);
        BizError::new(BizCode::JWT_INVALID)
    })?;

    Ok(SlimUser { uid: decoded.claims.sub })
}

pub fn jwt_from_header(headers: &HeaderMap) -> BizResult<String> {
    // 用于从请求头中获取JWT
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(BizError::new(BizCode::LOGIN_TIMEOUT)),
        // None => return Err(AuthError::NoAuthHeaderError),
    };

    let auth_header = header.to_str().map_err(|err| {
        error!("parse jwt from header. toStr error: {:?}", err);
        return BizError::new(BizCode::INVALID_AUTH_HEADER);
    })?;

    if !auth_header.starts_with(BEARER) {
        return Err(BizError::new(BizCode::INVALID_AUTH_HEADER));
    }

    info!("auth_header: {}", auth_header);
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}

pub fn authorize((role, headers): (Role, HeaderMap)) -> BizResult<String> {
    // 用于验证JWT的有效性和角色权限
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            info!("jwt: {:?}", jwt);
            let decoded = jsonwebtoken::decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| BizError::new(BizCode::JWT_INVALID))?;

            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(BizError::new(BizCode::NO_PERMISSION));
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(e),
    }
}
