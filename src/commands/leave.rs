
use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::connection::leave_call, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    leave_call(ctx, interaction).await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .description("leaves the call")
}
