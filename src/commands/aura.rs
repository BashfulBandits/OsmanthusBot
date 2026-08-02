
use serenity::{all::{Context, CreateCommand, Interaction}, model::connection};

use crate::{common::{connection, queue}, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = interaction.guild_id().unwrap();
    let channel_id = interaction.as_command().unwrap().channel_id;

    //connection::join_call(ctx, interaction).await?;

    let url = "https://www.youtube.com/watch?v=eIb1rSiTKOc".to_string();

    match queue::add_track_to_queue(ctx, &guild_id, &channel_id, url).await {
        Ok(_) => Ok(CommandSuccess::Play),
        Err(err) => Err(err),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("aura")
        .description("For True Aura Farmers")
}
