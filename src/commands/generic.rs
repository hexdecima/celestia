use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};

use super::SlashCommand;

pub fn ping() -> SlashCommand {
    SlashCommand::builder("ping")
        .with_description("HEARTBEAT PING?")
        .with_handler(|ctx, cmd| {
            Box::pin(async move {
                cmd.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("Pong."),
                    ),
                )
                .await
                .expect("failed to respond!");
            })
        })
        .build()
}
