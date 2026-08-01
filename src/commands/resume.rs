use serenity::all::{Context, CreateCommand, Interaction};

use crate::results::{CommandError, CommandSuccess};

pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = interaction.guild_id().unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler  = handler_lock.lock().await;
        handler.queue().resume();
        Ok(CommandSuccess::Resume)
    } else {
        return Err(CommandError::GetGuild);
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("resume").description("Resumes the current track")
}
