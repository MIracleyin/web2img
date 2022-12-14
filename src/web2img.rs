use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser, LaunchOptions};
use anyhow::{Result, Ok};
use image::{Luma, DynamicImage, load_from_memory, imageops::overlay, ImageFormat, GenericImageView};
use qrcode::QrCode;

fn url2img(url: &str) -> Result<DynamicImage>{
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
        // .find_element("body")?
        .capture_screenshot(CaptureScreenshotFormatOption::Png)?;
    Ok(load_from_memory(&data)?)

}

fn gen_qrcode(url: &str) -> Result<DynamicImage>{
    // Encode some data into bits.
    let code = QrCode::new(url.as_bytes()).unwrap();

    // Render the bits into an image.
    let buf = code.render::<Luma<u8>>().build();
    Ok(DynamicImage::ImageLuma8(buf))
    
}

fn do_overlay(botton: &mut DynamicImage, top: &DynamicImage){
    let x = botton.width() - top.width() - 10;
    let y = botton.height() - top.width() - 10;
    overlay(botton, top, x, y);
}

pub fn web2img(url: &str, output: &str, format:ImageFormat) -> Result<()> {
    let mut botton = url2img(url)?;
    let qrcode = gen_qrcode(url)?;
    do_overlay(&mut botton, &qrcode);
    botton.save_with_format(output, format)?;
    Ok(())
}