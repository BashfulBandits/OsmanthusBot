use serenity::all::standard::CommandResult;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId, Interaction, ResolvedValue};

pub async fn join_call(ctx: &Context, interaction: &Interaction) -> CommandResult {
    let guild_id = interaction.guild_id().unwrap();
    let user_id = interaction.clone().into_command().unwrap().user.id;

    let channel_id = {
        let guild = ctx.cache.guild(guild_id).expect("Guild not in chache");
        guild
            .voice_states
            .get(&user_id)
            .and_then(|voice_state| voice_state.channel_id)
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
        panic!("Join yay!")
    }
    
    Ok(())
}

pub async fn leave_call(ctx: &Context, interaction: &Interaction) -> CommandResult {
    let guild_id = interaction.guild_id().unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if manager.get(guild_id).is_none() {
        println!("Not currently in a voice channel");
        return Ok(());
    }

    manager.leave(guild_id).await?;
    println!("Left voice channel!");

    Ok(())
}
