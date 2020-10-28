use error_chain::error_chain;
use std::{env, fs::File, io::prelude::*, path::Path, process};

mod slicer;
use slicer::Slicer;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = get_program_arguments();
    let full_url = &*args[1];

    let response = reqwest::get(full_url).await?;
    assert!(response.status().is_success());

    let content = response.bytes().await?;

    let path = Path::new(Slicer::target_with_extension(full_url));

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(&content)?;
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
