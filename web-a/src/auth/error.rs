use thiserror::Error;
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("no permission")]
    NoPermissionError,

    #[error("Request failed: {0:?}")]
    Custom(#[from] anyhow::Error),

    #[error("Request failed: {0:?}")]
    DieselError(#[from] diesel::result::Error),
}
