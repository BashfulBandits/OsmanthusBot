
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, Interaction, ResolvedOption, ResolvedValue, standard::CommandResult};
use songbird::{Event, TrackEvent, input::{Compose, YoutubeDl}};

use crate::{HttpKey, TrackMetadata, common::{connection, queue}, events::{track_end::SongEndNotifier, track_start::SongStartNotifier}};


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    let guild_id = interaction.guild_id().unwrap();
    let _ = connection::join_call(ctx, interaction).await;

    let url = match options.first() {
        Some(ResolvedOption { value: ResolvedValue::String(url), .. }) => url.to_string(),
        _ => unreachable!(),
    };

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

        let src = YoutubeDl::new(http_client, url.clone());
        queue::add_track_metadata(ctx, guild_id, TrackMetadata { title: src.clone().aux_metadata().await.expect("").title.expect("") }).await;

        let track_handle = handler.enqueue_input(src.into()).await;
        let _ = track_handle.add_event(
            Event::Track(TrackEvent::End),
        SongEndNotifier {
                guild_id,
                channel_id: interaction.as_command().unwrap().channel_id, // adjust to however you get this in your code
                ctx: ctx.clone(),
                http: ctx.http.clone(),
            },
        );
        let _ = track_handle.add_event(
            Event::Track(TrackEvent::Play),
        SongStartNotifier {
                guild_id,
                channel_id: interaction.as_command().unwrap().channel_id, // adjust to however you get this in your code
                ctx: ctx.clone(),
                http: ctx.http.clone(),
            },
        );

        println!("queue: {:?}", handler.queue());
    } else {
        println!("Bot not in a voice call for this guild");
    }

    format!("Now Playing: {}", url)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("plays audio")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "link", "A youtube url to play").required(true))
}
