use futures::future::ready;
use futures::future::Ready;
use log::error;
use ntex::http::Payload;
use ntex::web::{Error, FromRequest, HttpRequest};

use crate::auth::error::AuthError;
use crate::auth::models::BaseUser;
use crate::common::consts::JWT_USER;
use crate::common::error::WebAppError;

pub type LoggedUser = BaseUser;
impl<Err> FromRequest<Err> for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.headers().get(JWT_USER) {
            None => {
                error!("未登录或Token过期");
                Err(WebAppError::AuthError(AuthError::NoPermissionError).into())
            }
            Some(user_str) => {
                let user_str = user_str.to_str().unwrap();
                serde_json::from_str::<LoggedUser>(&user_str).map_err(From::from)
            }
        })
    }
}
