use selthi::Select;
use thirtyfour::prelude::*;

use crate::{TRANSLATION, VIM_MODE};

pub async fn choose_season(driver: &WebDriver) -> WebDriverResult<()> {
    let language = TRANSLATION.get().unwrap();

    let season_items = driver.find_all(By::Css("div[data-season-id]")).await?;
    let mut season_opts: Vec<String> = Vec::new();

    for season in season_items {
        season_opts.push(season.inner_html().await?);
    }

    let season_opt = if season_opts.len() > 1 {
        get_season(season_opts).unwrap()
    } else {
        season_opts[0].to_string()
    };

    let season_btn_xpath = format!("//div[text()='{}']", season_opt);
    let season_element = driver.query(By::XPath(&season_btn_xpath)).first().await?;

    // we execute a js script to not be redirect to other page by the pop up
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![season_element.to_json()?],
        )
        .await
        .expect(language.click_season_err);

    Ok(())
}

fn get_season(seasons: Vec<String>) -> Result<String, ()> {
    let language = TRANSLATION.get().unwrap();
    let vim_mode = VIM_MODE.get().unwrap();
    print!("\x1B[2J\x1B[1;1H");

    let help_msg = format!("{} {}", language.total_season_misc_text, seasons.len());

    let seasons = seasons.iter().map(String::as_str).collect();

    let ans = Select::new(language.select_season_misc_text, seasons)
        .with_help_message(&help_msg)
        .with_page_size(25)
        .with_vim_mode(*vim_mode)
        .prompt();

    match ans {
        Some(opt) => Ok(opt.to_string()),
        None => Err(println!("{}", language.choose_season_err)),
    }
}
