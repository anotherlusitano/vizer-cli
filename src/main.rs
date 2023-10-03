use crate::cli::search_media::search_media;
use clap::{arg, Arg, Command};
use player::watch_media::watch_media;

mod cli;
pub mod media;
mod player;

static mut VIM_MODE: bool = false;

fn main() {
    let matches = Command::new("vizer-cli")
        .about("CLI tool to watch movies/series/animes in portuguese")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("vim")
                .short('v')
                .long("vim")
                .required(false)
                .num_args(0)
                .help("VIM Mode for the enthusiast")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("search")
                .about("Search for your content")
                .short_flag('s')
                .arg(arg!(<SEARCH> "The Search for media").num_args(1..))
                .arg_required_else_help(true),
        )
        .get_matches();

    if matches.get_flag("vim") {
        unsafe { VIM_MODE = true };
    }

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let media_name = sub_matches
                .get_many::<String>("SEARCH")
                .expect("required")
                .map(|v| v.as_str())
                .collect::<String>();

            if media_name.len() < 4 {
                // because the site only allows us to search more than 3 characters
                panic!("Sorry, your query needs to be at least 4 characters")
            }

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
