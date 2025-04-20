use std::env;

use commands::SlashCommand;
use serenity::{
    all::{Command, Context, EventHandler, GatewayIntents, Interaction, Ready},
    async_trait, Client,
};

mod commands;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Missing 'BOT_TOKEN' environment variable.");
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Failed building client.");

    if let Err(e) = client.start().await {
        println!("ERR: {e:?}");
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Ready as {}.", ready.user.display_name());

        for command in commands::list_commands()
            .into_iter()
            .map(SlashCommand::into_create_builder)
            .collect::<Box<[_]>>()
        {
            Command::create_global_command(&ctx.http, command)
                .await
                .expect("can't create global command.");
        }
    }
    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        match int {
            Interaction::Command(cmd) => {
                let command = commands::find_command(&cmd.data.name);

                match command {
                    Some(command) => command.call_with(ctx, cmd).await,
                    None => (),
                };
            }
            _ => () 
        }
    }
}
