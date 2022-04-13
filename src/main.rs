use std::env;

use dotenv;
use serenity::{
    async_trait,
    model::{channel::Embed, channel::Message, event::MessageUpdateEvent, gateway::Ready, id::ChannelId},
    prelude::*,
};

struct Handler;

async fn count_images_and_report(ctx: Context, channel_id: ChannelId, embeds: Vec<Embed>) {
    let c = embeds.iter().filter(|&e| e.image.is_some()).count();
    if c > 1 {
        if let Err(why) = channel_id.say(&ctx.http, format!("> ({} images)", c)).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        count_images_and_report(ctx, msg.channel_id, msg.embeds).await;
    }

    async fn message_update(&self, ctx: Context, event: MessageUpdateEvent) {
        // We only care about internal Discord updates
        if event.author.is_some() {
            return;
        }
        
        if let Some(embeds) = event.embeds {
            count_images_and_report(ctx, event.channel_id, embeds).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
