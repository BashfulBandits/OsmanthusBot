
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, Interaction, ResolvedOption, ResolvedValue};
use songbird::{Event, TrackEvent, input::{Compose, YoutubeDl}};

use crate::{common::{connection, queue}, results::{CommandError, CommandSuccess}};


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = interaction.guild_id().unwrap();
    let channel_id = interaction.as_command().unwrap().channel_id;

    connection::join_call(ctx, interaction).await?;

    let url = match options.first() {
        Some(ResolvedOption { value: ResolvedValue::String(url), .. }) => url.to_string(),
        _ => unreachable!(),
    };

    match queue::add_track_to_queue(ctx, &guild_id, &channel_id, url).await {
        Ok(_) => Ok(CommandSuccess::Play),
        Err(err) => Err(err),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("plays audio")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "link", "A youtube url to play").required(true))
}
