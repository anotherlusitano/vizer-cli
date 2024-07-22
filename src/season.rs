use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};

use crate::driver::click_element::click_element;

#[derive(Clone)]
pub struct Season {
    pub text: String,
    pub web_element: WebElement,
}

impl Season {
    pub async fn click_season(
        self,
        driver: &WebDriver,
        error_message: &str,
    ) -> WebDriverResult<()> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}
