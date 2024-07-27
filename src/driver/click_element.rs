use fantoccini::{elements::Element, error::CmdError, Client};
use serde_json::json;

/// This function is used when we need to click on a web element but there is a pop up in front of
/// it
pub async fn click_element(
    driver: &Client,
    element: Element,
    error_message: &str,
) -> Result<(), CmdError> {
    driver
        .execute(
            r#"
            arguments[0].click();
            "#,
            vec![json!(element)],
        )
        .await
        .expect(error_message);

    Ok(())
}
