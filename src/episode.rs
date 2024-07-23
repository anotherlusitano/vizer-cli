use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};

use crate::driver::click_element::click_element;

#[derive(Clone)]
pub struct Episode {
    pub text: String,
    pub img_path: Option<String>,
    pub web_element: WebElement,
}

impl Episode {
    pub async fn click_episode(
        self,
        driver: &WebDriver,
        error_message: &str,
    ) -> WebDriverResult<()> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}
