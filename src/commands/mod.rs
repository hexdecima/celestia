use std::{future::Future, pin::Pin};

use poise::serenity_prelude::{CommandInteraction, Context, CreateCommand};

pub mod generic;

pub const DEFAULT_DESCRIPTION_TEXT: &str = "No description provided.";

/// Lists all commands available.
pub fn list_commands() -> Vec<SlashCommand> {
    vec![generic::ping()]
}

/// Returns a command with given `name`, if any.
pub fn find_command(name: impl AsRef<str>) -> Option<SlashCommand> {
    match name.as_ref() {
        "ping" => Some(generic::ping()),
        _ => None,
    }
}

pub type HandlerFn = fn(Context, CommandInteraction) -> Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct SlashCommand {
    name: String,
    description: Option<String>,
    handler: HandlerFn,
}

impl SlashCommand {
    pub fn builder(name: impl AsRef<str>) -> SlashCommandBuilder<WithoutHandler> {
        SlashCommandBuilder {
            name: name.as_ref().to_owned(),
            description: None,
            state: WithoutHandler,
        }
    }
}

impl SlashCommand {
    /// Converts this into `serenity`'s command builder type.
    pub fn into_create_builder(self) -> CreateCommand {
        CreateCommand::new(self.name()).description(self.description())
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        if let Some(d) = &self.description {
            d
        } else {
            DEFAULT_DESCRIPTION_TEXT
        }
    }
    /// Calls this command with a given context and interaction.
    ///
    /// Returns a future.
    pub fn call_with(
        &self,
        ctx: Context,
        interaction: CommandInteraction,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        (self.handler)(ctx, interaction)
    }
}

pub struct SlashCommandBuilder<S: SlashCommandBuilderState> {
    name: String,
    description: Option<String>,
    state: S,
}

impl<S: SlashCommandBuilderState> SlashCommandBuilder<S> {
    pub fn with_description(self, description: impl AsRef<str>) -> Self {
        Self {
            description: Some(description.as_ref().to_owned()),
            ..self
        }
    }
}

impl SlashCommandBuilder<WithoutHandler> {
    /// Add a given closure as this command's handler.
    ///
    /// This method call may be followed by `.build()`.
    pub fn with_handler(self, handler: HandlerFn) -> SlashCommandBuilder<WithHandler> {
        SlashCommandBuilder {
            name: self.name,
            description: self.description,
            state: WithHandler { handler },
        }
    }
}

impl SlashCommandBuilder<WithHandler> {
    /// Finishes this builder, returning a command.
    pub fn build(self) -> SlashCommand {
        SlashCommand {
            name: self.name,
            description: self.description,
            handler: self.state.handler,
        }
    }
}

pub trait SlashCommandBuilderState {}
pub struct WithoutHandler;
pub struct WithHandler {
    handler: HandlerFn,
}
impl SlashCommandBuilderState for WithoutHandler {}
impl SlashCommandBuilderState for WithHandler {}
