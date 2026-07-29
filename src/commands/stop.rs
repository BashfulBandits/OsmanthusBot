use serenity::all::{Context, CreateCommand, Interaction};

use crate::common::{connection, queue};

pub async fn run(ctx: &Context, interaction: &Interaction) -> String {
    let guild_id = interaction.guild_id().unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        handler.queue().stop();
        let _ = connection::leave_call(ctx, interaction).await;
        queue::remove_all_track_metadata(ctx, guild_id).await;
        // clear queue data
        "Stopped and cleared the queue".to_string()
    } else {
        "Not in a voice channel".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop").description("Stops the track")
}
