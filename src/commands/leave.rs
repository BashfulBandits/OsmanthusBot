
use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::connection::leave_call, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    let guild_id = match interaction.guild_id() {
        Some(id) => id,
        None => return Err(CommandError::GetGuild),
    };

    println!("Leave call");
    leave_call(ctx, guild_id).await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .description("leaves the call")
}
