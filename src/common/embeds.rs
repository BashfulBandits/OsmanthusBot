use serenity::all::{Color, Context, CreateEmbed, GuildId};

use crate::{common::queue::get_track_metadata, results::CommandError};

pub static ERROR_COLOR: Color = Color::from_rgb(160, 50, 50);
pub static OKAY_COLOR: Color = Color::from_rgb(50, 200, 50);
pub static OSMANTHUS_COLOR: Color = Color::from_rgb(160, 100, 110);

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
            CommandError::InvalidURL => "Invalid URL\nPlease Try Again With Valid URL",
            CommandError::NotInCall => "Not In Call\nPlease Join a Call and Try Again",
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
        let seconds = track_duration % 60;

        CreateEmbed::new()
            .title(format!("Now Playing:   '{}'   |   {}:{}", first_track_metadata.title, minutes, seconds))
            .field("Queue", "IDK", true)
            .colour(OSMANTHUS_COLOR)
    } else {
        CreateEmbed::new()
    }
}

pub async fn added_to_queue_embed() -> CreateEmbed {
    CreateEmbed::new()
}
