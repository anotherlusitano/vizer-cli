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
                .arg(arg!(<SEARCH> "The Search for media").num_args(1..))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let media_name = sub_matches
                .get_many::<String>("SEARCH")
                .expect("required")
                .map(|v| v.as_str())
                .collect::<String>();

            match search_media(&media_name) {
                Ok(media_link) => {
                    watch_media(media_link).unwrap();
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        _ => println!("No Choice?"),
    }
}
