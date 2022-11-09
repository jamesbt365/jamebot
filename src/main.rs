

use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Cannot find envvar DISCORD_TOKEN");


    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
    .event_handler(Handler)
    .await
    .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
}