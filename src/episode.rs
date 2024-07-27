use fantoccini::{elements::Element, error::CmdError, Client};

use crate::driver::click_element::click_element;

#[derive(Clone)]
pub struct Episode {
    pub text: String,
    pub img_path: Option<String>,
    pub episode_number: usize,
    pub web_element: Element,
}

impl Episode {
    pub async fn click_episode(self, driver: &Client, error_message: &str) -> Result<(), CmdError> {
        click_element(driver, self.web_element, error_message).await?;
        Ok(())
    }
}
