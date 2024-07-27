use crate::driver::click_element::click_element;
use fantoccini::{elements::Element, error::CmdError, Client};

#[derive(Clone)]
pub struct Season {
    pub text: String,
    pub web_element: Element,
}

impl Season {
    pub async fn click_season(self, driver: &Client, error_message: &str) -> Result<(), CmdError> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}
