use error_chain::error_chain;
use std::{env, process};

mod downloader;
use downloader::BlockingDownloader;
mod decompressor;

mod slicer;

mod traits;
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
    let to_path = &(args[2].to_string() + &"/".to_string());

    let url_install = UrlInstall {
        downloader: Box::new(BlockingDownloader {}),
        decompressor: UrlInstall::get_decompressor(from_url),
    };
    url_install.run(from_url, to_path)?;

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
