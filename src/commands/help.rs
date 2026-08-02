
use serenity::all::{Context, CreateCommand, Interaction};

use crate::results::{CommandError, CommandSuccess};


pub async fn run(_ctx: &Context, _interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    println!("help");
    Ok(CommandSuccess::Help)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("Start Here!")
}
