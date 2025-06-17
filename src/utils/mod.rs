use poise::serenity_prelude::UserId;
use std::env;
use thiserror::Error;

const BOT_OWNERS: &str = "BOT_OWNERS";
const BOT_TOKEN: &str = "BOT_TOKEN";
/// Reads the ids declared in `BOT_OWNERS` and parse them into `UserId`s.
///
/// Fails if there aren't any or one of them is invalid (not a `u64`).
pub fn load_owners() -> Result<Box<[UserId]>, LoadEnvError> {
    env::var(BOT_OWNERS).map_or(
        Err(LoadEnvError::Missing("BOT_OWNERS".to_string())),
        |owners| {
            Ok(owners
                .split(',')
                .map(|id| {
                    id.parse::<u64>()
                        .map_err(|_| LoadEnvError::Malformatted(BOT_OWNERS.to_string()))
                })
                .collect::<Result<Box<_>, _>>()?
                .iter()
                .map(|id| UserId::new(*id))
                .collect())
        },
    )
}
/// Reads the token from `BOT_TOKEN`.
///
/// Fails if it is missing.
pub fn load_token() -> Result<String, LoadEnvError> {
    env::var(BOT_TOKEN).map_err(|_| LoadEnvError::Missing(BOT_TOKEN.to_string()))
}
#[derive(Debug, Error)]
pub enum LoadEnvError {
    #[error("Missing environment variable '{0}'")]
    Missing(String),
    #[error("'{0}' variable found, but it's malformatted.")]
    Malformatted(String),
}
