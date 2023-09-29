use crate::cli::search_media::search_media;
use clap::{arg, Command};
use player::watch_media::watch_media;

mod cli;
pub mod media;
mod player;

fn main() {
    let matches = Command::new("vizer-cli")
        .about("CLI tool to watch movies/series/animes in portuguese")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("search")
                .about("Search something")
                .arg(arg!(<SEARCH> "The Search for media"))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let media_name = sub_matches
                .get_one::<String>("SEARCH")
                .expect("required")
                .as_str();

            let media_link = search_media(media_name).unwrap();

            watch_media(media_link).unwrap();
        }
        _ => println!("No Choice?"),
    }
}
