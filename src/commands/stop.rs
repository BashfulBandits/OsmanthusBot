use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::{connection, queue}, results::{CommandError, CommandSuccess}};

pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = interaction.guild_id().unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        handler.queue().stop();
        let _ = connection::leave_call(ctx, interaction).await;
        queue::remove_all_track_metadata(ctx, guild_id).await;
        // clear queue data
        Ok(CommandSuccess::Stop)
    } else {
        Err(CommandError::GetGuild)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop").description("Stops the track")
}
