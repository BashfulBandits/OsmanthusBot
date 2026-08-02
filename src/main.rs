mod commands;
mod common;
mod events;
mod results;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};

use serenity::all::prelude::TypeMapKey;
use serenity::all::{Context, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, EventHandler, GatewayIntents, GuildId, Interaction, Ready};
use serenity::{Client, async_trait};
use songbird::SerenityInit;

use reqwest::Client as HttpClient;

use crate::common::embeds::{self, thinking_embed};
use crate::results::{CommandError, CommandSuccess};


struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}


#[derive(Clone)]
struct TrackMetadata {
    title: String,
    creator: String,
    duration: std::time::Duration,
    url: String,
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
            let data = CreateInteractionResponseMessage::new().embed(thinking_embed().await);
            let builder = CreateInteractionResponse::Message(data);
                
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }

            let command_result = match command.data.name.as_str() {
                "join" => Some(commands::join::run(&ctx, &interaction).await),
                "leave" => Some(commands::leave::run(&ctx, &interaction).await),

                "play" => Some(commands::play::run(&command.data.options(), &ctx, &interaction).await),
                "stop" => Some(commands::stop::run(&ctx, &interaction).await),
                "skip" => Some(commands::skip::run(&ctx, &interaction).await),
                "pause" => Some(commands::pause::run(&ctx, &interaction).await),
                "resume" => Some(commands::resume::run(&ctx, &interaction).await),
                _ => Some(Err(CommandError::CommandNotImplemented)),
            };

            let guild_id = interaction.guild_id().unwrap();

            let followup_response: EditInteractionResponse = match command_result {
                Some(Ok(CommandSuccess::Join))   => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Leave))  => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Pause))  => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Play))   => { EditInteractionResponse::new().add_embed(embeds::added_to_queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Resume)) => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Skip))   => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }
                Some(Ok(CommandSuccess::Stop))   => { EditInteractionResponse::new().add_embed(embeds::queue_embed(&ctx, &guild_id).await) }

                Some(Err(err))     => { EditInteractionResponse::new().add_embed(embeds::error_embed(err).await) }

                None => { println!("How????"); EditInteractionResponse::new().add_embed(embeds::error_embed(CommandError::Other).await)}
            };
            
            if let Err(why) = command.edit_response(&ctx.http, followup_response).await {
                println!("Cannot edit response: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_ids = vec![
            GuildId::new(env::var("TEST_GUILD_ID")
                .expect("Expected TEST_GUILD_ID in environment")
                .parse()
                .expect("TEST_GUILD_ID must be an integer")),
            GuildId::new(env::var("TEST_GUILD_ID_2")
                .expect("Expected TEST_GUILD_ID_2 in environment")
                .parse()
                .expect("TEST_GUILD_ID_2 must be an integer")),
        ];

        for guild_id in guild_ids {
            let _ = guild_id.set_commands(&ctx.http, vec![
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
