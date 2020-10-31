use error_chain::error_chain;
use std::{env, process};

mod downloader;
use downloader::BlockingDownloader;
mod decompressor;
use decompressor::{TarGzDecompressor, ZipDecompressor};

mod slicer;

mod traits;
use traits::Decompressor;
mod url_install;
use url_install::UrlInstall;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn main() -> std::io::Result<()> {
    let args = get_program_arguments();
    let from_url = &*args[1];

    let url_install = UrlInstall {
        downloader: Box::new(BlockingDownloader {}),
        decompressor: get_decompressor(from_url),
    };
    url_install.run(from_url)?;

    Ok(())
}

fn get_program_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: url-install <url1> <url2> .. <urln>");
        process::exit(0x0001);
    }
    args
}

fn get_decompressor(url: &str) -> Box<dyn Decompressor> {
    let decompressor: Box<dyn Decompressor> = if url.ends_with(".tar.gz") {
        Box::new(TarGzDecompressor {})
    } else if url.ends_with(".zip") {
        Box::new(ZipDecompressor {})
    } else {
        panic!("Unknown extension type.")
    };

    decompressor
}
