pub mod generic;

pub const DEFAULT_DESCRIPTION_TEXT: &str = "No description provided.";

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type CommandResult = Result<(), Error>;
