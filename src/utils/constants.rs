use std::{env as std_env, sync::LazyLock};

use dotenvy::dotenv;

pub const AWS_MAX_RETRIES: u32 = 10;
pub const REGION: &str = "eu-central-1";
pub const LOGGING_TABLE_NAME: &str = "logging";

pub mod env {
    pub const LOG_GROUP_NAME_ENV_VAR: &str = "LOG_GROUP_NAME";
}

pub static LOG_GROUP_NAME_SECRET: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    let secret =
        std_env::var(env::LOG_GROUP_NAME_ENV_VAR).expect("LOG_GROUP_NAME_ENV_VAR must be set.");
    if secret.is_empty() {
        panic!("LOG_GROUP_NAME_ENV_VAR must not be empty.");
    }
    secret
});
