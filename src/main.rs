use commands::Data;
use poise::{
    serenity_prelude::{async_trait, ClientBuilder, Context, EventHandler, GatewayIntents, Ready},
    Framework, FrameworkOptions,
};

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = utils::load_token().expect("Failed to load token.");

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![commands::generic::ping()],
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = ClientBuilder::new(&token, GatewayIntents::empty())
        .framework(framework)
        .event_handler(Handler)
        .await
        .unwrap();
    client.start().await.unwrap();
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Ready as {}.", ready.user.display_name());
    }
}
