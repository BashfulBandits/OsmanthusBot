mod commands;
mod common;
mod events;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};

use serenity::all::prelude::TypeMapKey;
use serenity::all::{ChannelId, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, GatewayIntents, GuildId, Http, Interaction, Ready};
use serenity::{Client, async_trait};
use songbird::{Event, EventContext, SerenityInit};

use reqwest::Client as HttpClient;


struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}


struct TrackMetadata {
    title: String,
}

struct AuxMetadataKey;
impl TypeMapKey for AuxMetadataKey {
    type Value = Arc<RwLock<HashMap<GuildId, Vec<TrackMetadata>>>>;
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(ref command) = interaction {
            let content = match command.data.name.as_str() {
                "join" => Some(commands::join::run(&command.data.options(), &ctx, &interaction).await),
                "leave" => Some(commands::leave::run(&command.data.options(), &ctx, &interaction).await),

                "play" => Some(commands::play::run(&command.data.options(), &ctx, &interaction).await),
                "stop" => Some(commands::stop::run(&ctx, &interaction).await),
                "skip" => Some(commands::skip::run(&ctx, &interaction).await),
                "pause" => Some(commands::pause::run(&ctx, &interaction).await),
                "resume" => Some(commands::resume::run(&ctx, &interaction).await),
                _ => Some("not implemented :(".to_string()),
            };
            println!("Pre Some() Content: {:?}\n", content);

            if let Some(content) = content {
                println!("In Some() Content: {:?}\n", content);
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId::new(
            env::var("TEST_GUILD_ID")
                .expect("Expected TEST_GUILD_ID in environment")
                .parse()
                .expect("TEST_GUILD_ID must be an integer")
        );

        let _commands = guild_id.set_commands(ctx.http, vec![
            commands::join::register(),
            commands::leave::register(),
            commands::play::register(),
            commands::stop::register(),
            commands::skip::register(),
            commands::pause::register(),
            commands::resume::register(),
        ]).await;
    }
}

#[tokio::main]
async fn main() {

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new()) // Add a queue data thing here
        .type_map_insert::<AuxMetadataKey>(Arc::new(RwLock::new(HashMap::new()))) // Add a queue data thing here
        .await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
