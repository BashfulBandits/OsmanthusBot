use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::connection::join_call, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    println!("Join call");
    join_call(ctx, interaction).await
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("joins the call")
}
