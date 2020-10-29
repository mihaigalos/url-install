use error_chain::error_chain;
use std::{env, process};

mod slicer;
use slicer::Slicer;

mod downloader;
use downloader::Downloader;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn main() -> std::io::Result<()> {
    let args = get_program_arguments();
    let full_url = &*args[1];

    Downloader::run(full_url, Slicer::target_with_extension(full_url))?;
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
