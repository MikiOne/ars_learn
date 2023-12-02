use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BizCode(&'static str);

impl BizCode {
    pub fn code(&self) -> &'static str {
        self.0
    }

    pub fn reason(&self) -> Option<&'static str> {
        canonical_reason(self.code())
    }

    pub fn code_reason(&self) -> String {
        format!("{}: {}", self.code(), self.reason().unwrap())
    }
}

macro_rules! biz_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl BizCode {
        $(
            $(#[$docs])*
            pub const $konst: BizCode = BizCode($num);
        )+

        }

        fn canonical_reason(num: &'static str) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

biz_codes! {
    ("000000", SUCCESS, "Success");
    ("000001", SYSTEM_ERROR, "System error");
    ("AU0001", WRONG_CREDENTIALS, "wrong credentials");
    ("AU0002", JWT_INVALID, "jwt token not valid");
    ("AU0003", JWT_CREATION_ERR, "jwt token creation error");
    ("AU0004", LOGIN_TIMEOUT, "Login timeout");
    ("AU0005", INVALID_AUTH_HEADER, "invalid auth header");
    ("AU0006", NO_PERMISSION, "no permission");
    ("AU0007", LOGOUT_SUCCESS, "Logout success");
    ("DB0001", DATABASE_ERROR, "Database error");
    ("ORM001", DIESEL_ERROR, "Diesel error");
    ("UR0001", USER_NOT_FOUND, "User not found");
}
