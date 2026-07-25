use serenity::all::{Context, CreateCommand, Interaction};

pub async fn run(ctx: &Context, interaction: &Interaction) -> String {
    let guild_id = interaction.guild_id().unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        handler.queue().pause();
        "Paused".to_string()
    } else {
        "Not in a voice channel".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pause").description("Pauses the current track")
}
