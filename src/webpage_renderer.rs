use std::fs;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page::CaptureScreenshotFormatOption};

pub fn render_page(source_file_path: &str) {
    let launch_options  = headless_chrome::LaunchOptions {
        headless: false,
       ..Default::default()
    };
    let browser = Browser::new(launch_options).expect("Failed to launch browser");
    let tab = browser.new_tab().expect("Failed to create new tab");
    tab.navigate_to(source_file_path);
    tab.wait_until_navigated();
    tab.capture_screenshot(headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png, None, None, false);
}

pub fn screenshot_thepage() -> Result<(), Box<dyn std::error::Error>> {
    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    let options  = headless_chrome::LaunchOptions {
        sandbox: false,
        enable_logging: true,
        headless: false,
        window_size: Some((1280, 800)),
        disable_default_args: true,
        ..Default::default()
    };
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    let jpeg_data = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(CaptureScreenshotFormatOption::Jpeg, Some(75), None, true)?;
    fs::write("screenshot.jpg", jpeg_data)?;

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let png_data = tab
        .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(CaptureScreenshotFormatOption::Png)?;
    fs::write("screenshot.png", png_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screenshot_local() {
        let thepage= screenshot_thepage();
        assert!(thepage.is_ok(), "Failed to take screenshot of the local page");
    }
}