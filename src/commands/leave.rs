
use serenity::all::{Context, CreateCommand, Interaction, ResolvedOption};

use crate::commands::connection::leave_call;


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    match leave_call(ctx, interaction).await {
        Ok(_) => "OsBot has left the call".to_string(),
        Err(_) => "OsBot is not in a call".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .description("leaves the call")
}
