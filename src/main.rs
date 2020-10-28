use error_chain::error_chain;
use std::{env, fs::File, io::Write, path::Path, process};

mod slicer;
use slicer::Slicer;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn write_file(full_url: &str, response: reqwest::blocking::Response) -> std::io::Result<()> {
    let content = response.bytes().unwrap();
    let target_with_extension = Path::new(Slicer::target_with_extension(full_url));

    File::create(&target_with_extension)
        .expect("Unable to create file")
        .write_all(&content)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = get_program_arguments();
    let full_url = &*args[1];

    let response = reqwest::blocking::get(full_url).unwrap();
    // assert!(response.status().is_success());
    write_file(full_url, response)?;
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
