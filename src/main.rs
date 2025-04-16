use std::env;

use serenity::{all::{Context, EventHandler, GatewayIntents, Ready}, async_trait, Client};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = env::var("BOT_TOKEN")
        .expect("Missing 'BOT_TOKEN' environment variable.");
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
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready as {}.", ready.user.display_name());
    }
}
