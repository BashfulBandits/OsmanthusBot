
use serenity::all::{Context, CreateCommand, Interaction, ResolvedOption};

use crate::commands::connection::leave_call;


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    println!("{:?}", leave_call(ctx, interaction).await);

    format!("you've been pinged!")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave")
        .description("leaves the call")
}
