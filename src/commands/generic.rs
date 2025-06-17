use poise::command;

use crate::commands::CommandResult;

use super::Context;

#[command(slash_command, description_localized("en-US", "HEARTBEAT PING?"))]
pub async fn ping(ctx: Context<'_>) -> CommandResult {
    ctx.say("Pong.").await?;
    Ok(())
}
