use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, ResolvedValue};
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    let name = match options.first().map(|o| &o.value) {
        Some(ResolvedValue::String(name)) => *name,
        _ => "Pong",
    };

    format!("{name}, you've been pinged!")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("A ping command")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "name", "name to ping"))
}
