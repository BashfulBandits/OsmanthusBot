use std::fmt::format;

use serenity::all::{Color, Context, CreateEmbed, GuildId};

use crate::{common::queue::{self, get_track_metadata}, results::CommandError};

pub static ERROR_COLOR: Color = Color::from_rgb(160, 50, 50);
pub static OKAY_COLOR: Color = Color::from_rgb(50, 200, 50);
pub static OSMANTHUS_COLOR: Color = Color::from_rgb(130, 110, 110);

pub async fn thinking_embed() -> CreateEmbed {
    let thinking_embed = CreateEmbed::new().description("Thinking...");
        
    thinking_embed
}

pub async fn error_embed(error: CommandError) -> CreateEmbed {
    CreateEmbed::new()
        .title("Error")
        .colour(ERROR_COLOR)
        .description(match error {
            CommandError::AgeRestriction => "Video is age-restriced",
            CommandError::InvalidURL => "Invalid URL\nPlease Try Again With Valid URL\nIf url is valid, then it is likly the url is age-restriced",
            CommandError::NotInCall => "Not In Call\nPlease Join a Call and Try Again",
            CommandError::BotNotInCall => "Bot Not In Call\nTry Joining a Call and Trying Again",
            CommandError::GetGuild => "Fail in backend\nCreate Issue on GitHub if Persists",
            CommandError::CommandNotImplemented => "Command Not Implemented\n",
            CommandError::Other => "Unknown Error",
        })
}


pub async fn queue_embed(ctx: &Context, guild_id: &GuildId) -> CreateEmbed {
    let track_metadata = get_track_metadata(ctx, guild_id).await;

    if let Some(first_track_metadata) = track_metadata.first() {
        let track_duration = first_track_metadata.duration.as_secs();
        let minutes = track_duration / 60;
        let seconds_int = track_duration % 60;
        let seconds = if seconds_int == 0 { String::from("00") } else if seconds_int < 10 { format!("0{seconds_int}") } else { seconds_int.to_string() };

        let mut embed = CreateEmbed::new()
            .title("Now Playing")
            .description(format!("[{}]({})", first_track_metadata.title, first_track_metadata.url)) // Add 55ish character limit
            .field("Creator", first_track_metadata.creator.to_string(), true)
            .field("Duration", format!("{minutes}:{seconds}"), true)
            .colour(OSMANTHUS_COLOR);

        if track_metadata.len() > 1 {
            let next_up = track_metadata.iter()
                .skip(1)
                .enumerate()
                .map(|(i, t)| format!("{} - [{}]({})\n", i + 1, t.title, t.url))
                .collect::<String>();
            embed = embed.field("Next Up", next_up, false);
        }

        embed
    } else {
        CreateEmbed::new()
            .title("Queue is Empty!")
            .description("Play Another Song!")
            .colour(OSMANTHUS_COLOR)
    }
}

pub async fn added_to_queue_embed(ctx: &Context, guild_id: &GuildId) -> CreateEmbed {
    let queue_metadata = queue::get_track_metadata(ctx, guild_id).await;

    CreateEmbed::new()
        .colour(OKAY_COLOR)
        .title("Added to Queue".to_string())
        .description(queue_metadata.last().unwrap().title.clone())
}
