use thirtyfour::{error::WebDriverResult, WebDriver, WebElement};

/// This function is used when we need to click on a web element but there is a pop up in front of
/// it
pub async fn click_element(
    driver: &WebDriver,
    element: WebElement,
    error_message: &str,
) -> WebDriverResult<()> {
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![element.to_json()?],
        )
        .await
        .expect(error_message);

    Ok(())
}
