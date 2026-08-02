
use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::connection::join_call, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    println!("help");
    Ok(CommandSuccess::Help)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("Start Here!")
}
