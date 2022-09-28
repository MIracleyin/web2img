use std::fs;

use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};
use anyhow::Result;

fn url2img(url: &str) -> Result<Vec<u8>>{
    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let data = tab
        .navigate_to(url)?
        .wait_for_element("body")?
        .capture_screenshot(CaptureScreenshotFormatOption::Png)?;
    Ok(data)
}

pub fn web2img(url: &str, output: &str) -> Result<()> {
    let data = url2img(url)?;
    fs::write(output, &data)?;
    Ok(())
}