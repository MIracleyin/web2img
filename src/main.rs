use std::{path::Path, ffi::OsStr};

use anyhow::Result;
use clap::Parser;
use url::Url;

mod web2img;
use web2img::web2img;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(name = "Web2img")]
#[clap(author = "Yin Zhang <miracleyin@live.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "snapshot web homepage", long_about = None)]
#[clap(setting = clap::AppSettings::ColoredHelp)]
struct Args {
    /// image output path
    #[clap(short, long, default_value = "/tmp/snapshot.png", validator = valid_output_path)]
    output: String,
    /// url to capture
    #[clap(validator = valid_url)]
    url: String,
}

fn get_image_format(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|p| OsStr::to_str(p))
        .and_then(|ext| {
            let ext = ext.to_lowercase();
            match ext.as_str() {
                "png" | "jpg" | "jpeg" => Some(ext),
                _ => None,
            } 
        })
}

fn valid_url(url: &str) -> Result<(), String> {
    match Url::parse(url) {
        Ok(_) => Ok(()),
        Err(_) => Err("You must provide a valid url".into()),
    }
}

/// "tmp/abc.pdf" => 'tmp' exists, pdf (png | jpg, jpeg)
fn valid_output_path(name: &str) -> Result<(), String> {
    let path = Path::new(&name);
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    let ext = get_file_ext(path);

    if parent.is_none() {
        return Err("File path mast be exists".into());
    }

    if ext.is_none() {
        return Err("File ext must be png | jpg | jpeg".into());
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);

    web2img(&args.url, &args.output).unwrap();



    //    for _ in 0..args.count {
    //        println!("Hello {}!", args.name)
    //    }
}
