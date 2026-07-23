
use serenity::all::{Context, CreateCommand, Interaction, ResolvedOption, standard::CommandResult};
use songbird::input::File;


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    let guild_id = interaction.guild_id().unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = File::new("/home/ontos/coding/OsmanthusBot/input.mp3");

        let _ = handler.play_input(src.into());
    } else {
        println!("Bot not in a voice call for this guild");
    }

    "Playing".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("plays audio")
}
