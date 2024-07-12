use thirtyfour::prelude::*;

use crate::{
    cli::{
        choose_episode::choose_episode, choose_media::choose_media, choose_season::choose_season,
        get_media_name_from_user::get_media_name_from_user, get_media_url::get_media_url,
        get_medias::get_medias, get_video_url::get_video_url, menu::menu,
    },
    fs::posters::get_posters_path,
    media::Media,
    player::play_video::play_video,
    TRANSLATION,
};

pub async fn watch_media(media: Media, img_mode: bool, driver: &WebDriver) -> WebDriverResult<()> {
    let language = TRANSLATION.get().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", language.preparing_misc_text);

    let url = format!("https://vizer.in/{}", &media.url);
    driver.goto(url).await?;

    if media.url.contains("serie/") {
        choose_season(driver).await?;

        println!("{}", language.getting_episodes_misc_text);

        choose_episode(driver, img_mode).await?;
    }

    let media_url = get_media_url(driver).await?;

    let mut video_url = get_video_url(driver, media_url).await?;

    play_video(&video_url);

    loop {
        match menu() {
            Ok("replay") => play_video(&video_url),
            Ok("quit") => break,
            Ok("search") => {
                let mut posters_path: Vec<String> = Vec::new();

                let media_name = get_media_name_from_user().unwrap();

                let medias = get_medias(&media_name).await;

                if medias.is_empty() {
                    eprintln!("{}", language.media_name_is_empty_exit_text);
                    break;
                }

                if img_mode {
                    let medias_poster_url: Vec<String> = medias
                        .clone()
                        .into_iter()
                        .map(|media| media.poster_url)
                        .collect();

                    posters_path = get_posters_path(medias_poster_url).await.unwrap();
                }
                match choose_media(medias, img_mode, posters_path) {
                    Ok(media) => {
                        let url = format!("https://vizer.in/{}", &media.url);
                        driver.goto(url).await?;

                        if media.url.contains("serie/") {
                            choose_season(driver).await?;

                            println!("{}", language.getting_episodes_misc_text);

                            choose_episode(driver, img_mode).await?;
                        }

                        let media_url = get_media_url(driver).await?;

                        video_url = get_video_url(driver, media_url).await?;

                        play_video(&video_url);
                    }
                    Err(err) => {
                        eprintln!("{:?}", err);
                        break;
                    }
                }
            }
            Err(err) => {
                eprint!("{:?}", err);
                break;
            }
            _ => break,
        }
    }

    Ok(())
}
