mod commands;

use std::env;

use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::futures::prelude;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "download" => Some(commands::download::run(&command.data.options())),
                "join" => Some(commands::join::run(&command.data.options(), &ctx, &command).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
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
            commands::ping::register(),
            commands::download::register(),
            commands::join::register(),
        ]).await;
    }
}

#[tokio::main]
async fn main() {
    get_fnne;
    
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
