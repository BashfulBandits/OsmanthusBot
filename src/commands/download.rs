use std::process::Command;

use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, ResolvedValue};
use serenity::model::application::ResolvedOption;

//use tokio::process::Command;

pub fn run(options: &[ResolvedOption]) -> String {
    let link = match options.first().map(|o| &o.value) {
        Some(ResolvedValue::String(link)) => *link,
        _ => unreachable!(),
    };

    //let download_command = format!("yt-dlp -x --audio-format mp3 --audio-quality 0 {}", link);

    let download_success = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format").arg("mp3")
        .arg("--audio-quality").arg("0")
        .arg(link)
        .output()
        .expect("failed");

    format!("Download status: {:?}", download_success)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("download")
        .description("Download a youtube videos audio")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "link", "link to video").required(true))
}
