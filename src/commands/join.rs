use serenity::all::{Context, CreateCommand, Interaction, ResolvedOption};

use crate::commands::connection::join_call;


pub async fn run(options: &[ResolvedOption<'_>], ctx: &Context, interaction: &Interaction) -> String {
    println!("join: {:?}", join_call(ctx, interaction).await);

    format!("you've been pinged!")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("joins the call")
}
