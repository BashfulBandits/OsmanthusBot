use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::{connection, queue}, results::{CommandError, CommandSuccess}};

pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = match interaction.guild_id() {
        Some(id) => id,
        None => return Err(CommandError::GetGuild),
    };

    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        
        handler.queue().stop();
        println!("Stoped");

        queue::remove_all_track_metadata(ctx, guild_id).await;
        println!("Queue remove");

        //connection::leave_call(ctx, guild_id).await?;
        //println!("Leave call");
        
        Ok(CommandSuccess::Stop)
    } else {
        Err(CommandError::GetGuild)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop").description("Stops the Playing and Clears the Queue")
}
