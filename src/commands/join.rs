use serenity::all::standard::CommandResult;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, Interaction, ResolvedValue};
use serenity::model::application::ResolvedOption;

pub async fn join_call(ctx: &Context, interaction: &CommandInteraction) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = interaction.guild(&ctx.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            //check_msg(msg.reply(ctx, "Not in a voice channel").await);
            println!("Not in a voice call");

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        //let mut handler = handler_lock.lock().await;
        //handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        panic!("Join failed")
    }
    
    Ok(())
}

pub async fn run(options: &[ResolvedOption], ctx: &Context, interaction: &CommandInteraction) -> String {
    println!("{:?}", join_call(ctx, interaction).await);

    format!("{name}, you've been pinged!")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("joins the call")
}
