use thirtyfour::prelude::*;

use crate::{
    cli::{
        choose_episode::choose_episode, choose_episode_with_images::choose_episode_with_images,
        choose_lang::choose_lang, choose_season::choose_season,
    },
    driver::start_driver::{get_driver, start_browser_driver},
    fs::posters::get_posters_path,
    media::Media,
    player::{mpv::open_mpv, vlc::open_vlc},
    TRANSLATION, USE_MPV,
};

#[tokio::main]
pub async fn watch_media(media: Media, img_mode: bool) -> WebDriverResult<()> {
    let language = TRANSLATION.get().unwrap();
    let use_mpv = USE_MPV.get().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!("{}", language.preparing_misc_text);

    let mut browser_driver = start_browser_driver();

    let driver = get_driver().await;

    let url = format!("https://vizer.in/{}", &media.url);
    driver.goto(url).await?;

    if media.url.contains("serie/") {
        choose_season(&driver).await?;

        println!("{}", language.getting_episodes_misc_text);

        let episodes_list = driver.find(By::ClassName("episodes")).await?;

        let episodes_items = episodes_list.query(By::ClassName("item")).all().await?;

        let mut episodes_opt: Vec<String> = Vec::new();
        let mut episodes_img_url: Vec<String> = Vec::new();

        for (i, item) in episodes_items.iter().enumerate() {
            if item.class_name().await?.unwrap() != "item unreleased " {
                let episode_text = item.find(By::Tag("span")).await?.inner_html().await?;
                // this thing of adding by 1
                // is just to show the episodes starting in 1
                let episode: String = format!("{} - {}", i + 1, episode_text);

                episodes_opt.push(episode);

                if img_mode {
                    let img_src = item.find(By::Tag("img")).await?.attr("src").await?.unwrap();
                    let img_url =
                        format!("https://vizertv.in{}", img_src.replace("s/185", "s/500"));
                    episodes_img_url.push(img_url);
                }
            }
        }

        let episode_opt: usize = if episodes_opt.len() > 1 {
            match img_mode {
                true => {
                    let posters_path = get_posters_path(episodes_img_url).await.unwrap();

                    choose_episode_with_images(episodes_opt, posters_path).unwrap()
                }
                false => choose_episode(episodes_opt).unwrap(),
            }
        } else {
            episodes_opt[0].parse::<usize>().unwrap() - 1
        };

        // we execute a js script to not be redirect to other page by the pop up
        driver
            .execute(
                r#"
            arguments[0].click();
            "#,
                vec![episodes_items[episode_opt].to_json()?],
            )
            .await
            .expect(language.click_episode_err);
    }

    println!("{}", language.getting_language_misc_text);

    let langs_items = driver.query(By::Css("div[data-audio]")).all().await?;

    let mut langs_opts: Vec<String> = Vec::new();

    for lang in &langs_items {
        let opt = lang
            .attr("data-audio")
            .await?
            .expect(language.language_option_expect);
        langs_opts.push(opt);
    }

    let lang_opt = if langs_opts.len() == 2 {
        choose_lang(langs_opts.clone()).unwrap()
    } else {
        langs_opts[0].to_string()
    };

    let mut media_id: Option<String> = None;
    for i in 0..langs_opts.len() {
        if langs_opts[i] == lang_opt {
            media_id = langs_items[i].attr("data-load-player").await?;
        }
    }

    println!("{}", language.fetching_misc_text);

    let media_url = format!(
        "https://vizer.in/embed/getEmbed.php?id={}&sv=mixdrop",
        media_id.unwrap()
    );

    driver.goto(media_url).await?;

    driver.enter_frame(0).await?;

    let play_button = driver
        .query(By::ClassName("vjs-big-play-button"))
        .first()
        .await?;

    // we execute a js script to not be redirect to other page by the pop up
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![play_button.to_json()?],
        )
        .await?;

    let video = driver.find(By::Id("videojs_html5_api")).await?;

    let video_url = format!("https:{}", video.attr("src").await?.unwrap());

    driver.quit().await?;
    browser_driver.kill().unwrap();

    if *use_mpv {
        open_mpv(&video_url);
    } else {
        open_vlc(&video_url);
    }

    Ok(())
}
