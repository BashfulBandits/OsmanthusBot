
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedOption, ResolvedValue, standard::CommandResult};
use songbird::input::YoutubeDl;

use crate::HttpKey;


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    let guild_id = interaction.guild_id().unwrap();

    let url = match options.first() {
        Some(ResolvedOption { value: ResolvedValue::String(url), .. }) => url.to_string(),
        _ => unreachable!(),
    };
    println!("URL: {}", url);

    let http_client = {
        let data = ctx.data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();


    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = YoutubeDl::new(http_client, url);

        let _ = handler.play_input(src.into());
    } else {
        println!("Bot not in a voice call for this guild");
    }

    "Playing".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("plays audio")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "link", "A youtube url to play").required(true))
}
