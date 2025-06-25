use std::error::Error;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{Browser};

pub fn render(local_route: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("creating browser");
    let browser = Browser::default()?;

    println!("creating tab");
    println!("navigating to: {}", local_route);
    let tab = browser.new_tab_with_options(CreateTarget {
        url: String::from(local_route),
        height: Some(600),
        width: Some(900),
        browser_context_id: None,
        enable_begin_frame_control: None,
        new_window: Some(true),
        background: None,
        for_tab: None,
    })?;

    println!("waiting for content element");
    let viewport = tab
        .wait_for_element("#content")?
        .get_box_model()?
        .margin_viewport();

    // Take a screenshot of the entire browser window
    println!("taking picture");
    let picture_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;
    println!("picture taken");
    Ok(picture_data)
}

#[cfg(test)]
mod chrometests {
    //use crate::html_renderer::render;

    #[test]
    fn hejsa() {
        // TODO: fix test
        assert!(true)
    }
}
