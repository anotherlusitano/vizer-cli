use std::{process::exit, sync::OnceLock};

use crate::{
    cli::get_medias::get_medias,
    fs::temp_dir::{create_temp_dir, remove_temp_dir},
};
use clap::{arg, Arg, Command};
use cli::choose_media::choose_media;
use fs::posters::get_posters_path;
use language::{get_translation, Translations};
use player::watch_media::watch_media;
use scraper::is_offline::is_offline;

mod cli;
mod driver;
pub mod episode;
mod fs;
pub mod language;
pub mod media;
mod player;
mod scraper;
pub mod season;

static TRANSLATION: OnceLock<Translations> = OnceLock::new();
static VIM_MODE: OnceLock<bool> = OnceLock::new();
static USE_MPV: OnceLock<bool> = OnceLock::new();
static USE_GECKODRIVER: OnceLock<bool> = OnceLock::new();

#[tokio::main]
async fn main() {
    let matches = Command::new("vizer-cli")
        .about("CLI tool to watch movies/series/animes in portuguese")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("english")
                .short('e')
                .long("english")
                .required(false)
                .num_args(0)
                .help("Change all the texts in the app to english")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("mpv")
                .short('m')
                .long("mpv")
                .required(false)
                .num_args(0)
                .help("Use MPV media player instead of VLC")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("geckodriver")
                .short('g')
                .long("geckodriver")
                .required(false)
                .num_args(0)
                .help("Use geckodriver instead of chromedriver")
                .action(clap::ArgAction::SetTrue),
        )
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

    if matches.get_flag("vim") {
        VIM_MODE.get_or_init(|| true);
    } else {
        VIM_MODE.get_or_init(|| false);
    }

    let mut img_mode = false;
    if matches.get_flag("img") {
        img_mode = true;
    }

    if matches.get_flag("english") {
        TRANSLATION.get_or_init(|| get_translation("english"));
    } else {
        TRANSLATION.get_or_init(|| get_translation("portuguese"));
    }

    if matches.get_flag("geckodriver") {
        USE_GECKODRIVER.get_or_init(|| true);
    } else {
        USE_GECKODRIVER.get_or_init(|| false);
    }

    if matches.get_flag("mpv") {
        USE_MPV.get_or_init(|| true);
    } else {
        USE_MPV.get_or_init(|| false);
    }

    let language = TRANSLATION.get().unwrap();

    if is_offline().await {
        eprintln!("{}", language.is_currently_offline);
        exit(1)
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
                eprintln!("{}", language.media_name_len_exit_text);
                exit(1)
            }

            let medias = get_medias(&media_name).await;

            if medias.is_empty() {
                eprintln!("{}", language.media_name_is_empty_exit_text);
                exit(1)
            }

            let mut posters_path: Vec<String> = Vec::new();

            if img_mode {
                create_temp_dir();
                let medias_poster_url: Vec<String> = medias
                    .clone()
                    .into_iter()
                    .map(|media| media.poster_url)
                    .collect();

                posters_path = get_posters_path(medias_poster_url).await.unwrap();
            }
            match choose_media(medias, img_mode, posters_path) {
                Ok(media) => {
                    watch_media(media, img_mode).await.unwrap();
                    if img_mode {
                        remove_temp_dir();
                    }
                }
                Err(err) => {
                    eprintln!("{:?}", err);

                    if img_mode {
                        remove_temp_dir();
                    }
                }
            }
        }
        _ => println!("{}", language.no_choice_misc_text),
    }
}
