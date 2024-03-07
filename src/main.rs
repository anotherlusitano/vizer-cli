use crate::{
    cli::get_medias::get_medias,
    fs::temp_dir::{create_temp_dir, remove_temp_dir},
};
use clap::{arg, Arg, Command};
use cli::{choose_media::choose_media, choose_with_images::choose_with_images};
use fs::posters::get_posters_path;
use player::watch_media::watch_media;
use tokio::runtime::Runtime;

mod cli;
mod fs;
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
        .arg(
            Arg::new("img")
                .short('i')
                .long("image-preview")
                .required(false)
                .num_args(0)
                .help("Enable you to see the posters as you choose them")
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

    let mut img_mode = false;
    if matches.get_flag("vim") {
        unsafe { VIM_MODE = true };
    } else if matches.get_flag("img") {
        img_mode = true;
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

            let medias = get_medias(&media_name);

            if medias.is_empty() {
                panic!("Couldn't find anything with your query")
            }

            if img_mode {
                create_temp_dir();
                let rt = Runtime::new().unwrap();
                let future = get_posters_path(medias.clone());
                let posters_path = rt.block_on(future).unwrap();
                let medias_title: Vec<String> = medias
                    .clone()
                    .into_iter()
                    .map(|media| media.title)
                    .collect();

                match choose_with_images(&medias_title, posters_path) {
                    Ok(media_index) => {
                        remove_temp_dir();
                        watch_media(medias[media_index].clone()).unwrap();
                    }
                    Err(err) => {
                        remove_temp_dir();
                        eprintln!("{:?}", err);
                    }
                }
            } else {
                match choose_media(medias) {
                    Ok(media) => {
                        watch_media(media).unwrap();
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
        _ => println!("No Choice?"),
    }
}
