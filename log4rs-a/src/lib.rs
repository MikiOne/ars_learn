pub mod date_util;


use anyhow::anyhow;
use once_cell::sync::OnceCell;

static GLOBAL_ENV: OnceCell<Env> = OnceCell::new();

#[derive(Debug, strum_macros::Display)]
pub enum Env {
    #[strum(serialize = "dev")]
    Dev,
    #[strum(serialize = "prod")]
    Prod,
}

impl Env {
    pub fn init_env<F>(env: F)
        where
            F: FnOnce() -> Env + 'static,
    {
        GLOBAL_ENV.get_or_init(env);
    }

    pub fn is_prod() -> anyhow::Result<bool> {
        if let Env::Prod = Self::env()? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn env() -> anyhow::Result<&'static Env> {
        match GLOBAL_ENV.get() {
            None => Err(anyhow!("Environment variable is not configured")),
            Some(env) => Ok(env),
        }
    }
}

