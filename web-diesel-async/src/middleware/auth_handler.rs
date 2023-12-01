use futures::future::ready;
use futures::future::Ready;
use log::error;
use ntex::http::Payload;
use ntex::web::{Error, FromRequest, HttpRequest};

use crate::auth::models::SlimUser;
use crate::common::biz_code::BizCode;
use crate::common::biz_error::BizError;
use crate::common::consts::JWT_USER;

pub type LoggedUser = SlimUser;
impl<Err> FromRequest<Err> for LoggedUser {
    type Error = BizError;
    type Future = Ready<Result<LoggedUser, BizError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.headers().get(JWT_USER) {
            None => {
                error!("未登录或Token过期");
                Err(BizError::new(BizCode::LOGIN_TIMEOUT))
            }
            Some(user_str) => {
                let user_str = user_str.to_str().unwrap();
                serde_json::from_str::<LoggedUser>(&user_str).map_err(|err| {
                    error!("Get LoggedUser from_request error: {:?}", err);
                    BizError::new(BizCode::LOGIN_TIMEOUT)
                })
            }
        })
    }

    // fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    //     ready(match req.headers().get(JWT_USER) {
    //         None => {
    //             error!("未登录或Token过期");
    //             Err(AuErr(BizCode::LOGIN_TIMEOUT).into())
    //         }
    //         Some(user_str) => {
    //             let user_str = user_str.to_str().unwrap();
    //             serde_json::from_str::<LoggedUser>(&user_str)
    //                 .map_err(|err| AuErr(BizCode::LOGIN_TIMEOUT).into())
    //         }
    //     })
    // }
}
