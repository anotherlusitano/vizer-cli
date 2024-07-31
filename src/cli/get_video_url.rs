use fantoccini::{error::CmdError, Client, Locator};

use crate::{
    cli::choose_lang::choose_lang, driver::click_element::click_element, language::Translations,
    TRANSLATION,
};

pub async fn get_video_url(driver: &Client) -> Result<String, CmdError> {
    let language = TRANSLATION.get().unwrap();
    println!("{}", language.getting_language_misc_text);

    driver
        .wait()
        .for_element(Locator::Css("div[data-audio]"))
        .await?;
    let langs_items = driver.find_all(Locator::Css("div[data-audio]")).await?;

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

    for lang in langs_opts {
        if lang == lang_opt {
            let lang_css_selector = format!("div[data-audio='{}']", lang_opt);

            driver
                .find(Locator::Css(&lang_css_selector))
                .await?
                .click()
                .await?;
            break;
        }
    }

    println!("{}", language.fetching_misc_text);

    let mixdrop_btn = driver
        .find(Locator::Css("div[data-load-embed-server='mixdrop']"))
        .await?;

    click_element(driver, mixdrop_btn, language.language_option_expect).await?;

    let url = get_url(driver, language).await?;

    Ok(url)
}

async fn get_url(driver: &Client, language: &Translations) -> Result<String, CmdError> {
    // NOTE: We have to use two enter_frame instead of enter_frame(1)
    // because the video don't load fast enough
    // and we need to specify which frame to enter
    // because firefox handles iframes differently from chromium
    driver
        .wait()
        .for_element(Locator::Css("iframe[src^='embed']"))
        .await?;
    let player_div = driver.find(Locator::Css("iframe[src^='embed']")).await?;
    player_div.enter_frame().await?;

    driver.wait().for_element(Locator::Css("iframe")).await?;
    driver.enter_frame(Some(0)).await?;

    // we wait for the elements inside the iframe to appear
    driver.wait().for_element(Locator::Css("video")).await?;

    driver.wait().for_element(Locator::Id("videojs")).await?;
    driver
        .wait()
        .for_element(Locator::Css(".vjs-big-play-button"))
        .await?;

    // NOTE: We use this loop to ensure that we click on the button so that the src of the video appears
    loop {
        match driver.find(Locator::Css("video[src]")).await {
            Ok(_) => {
                break;
            }
            Err(_) => {
                // NOTE: We find the button here so we don't get the
                // StaleElementReferenceException error
                let play_button = driver.find(Locator::Css(".vjs-big-play-button")).await?;

                click_element(driver, play_button.clone(), language.click_play_button_err).await?;
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }
    }

    let video = driver.find(Locator::Css("video[src]")).await?;

    let video_src = video.attr("src").await?.unwrap();

    // NOTE: We use this to leave the iframes so we can do some action later
    driver.enter_parent_frame().await?;
    driver.enter_parent_frame().await?;

    let video_url = format!("https:{}", video_src);
    Ok(video_url)
}
