
use serenity::all::{Context, CreateCommand, Interaction};

use crate::{common::connection::join_call, results::{CommandError, CommandSuccess}};


pub async fn run(ctx: &Context, interaction: &Interaction) -> Result<CommandSuccess, CommandError> {
    println!("aura");
    Ok(CommandSuccess::Aura)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("aura")
        .description("For True Aura Farmers")
}
