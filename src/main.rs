use error_chain::error_chain;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::App;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn target_with_extension(s: &str) -> &str {
    let pos_of_last_slash = s.rfind('/').unwrap();
    &s[pos_of_last_slash + 1..]
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("url-install")
        .arg("<output> 'Sets an optional output file'")
        .get_matches();

    let mut target = ""; // https://github.com/Byron/dua-cli/releases/download/v2.10.2/dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz

    if let Some(o) = matches.value_of("output") {
        target = o;
    }

    let _name_with_extension = target_with_extension(&target).to_string();
    println!("{}", _name_with_extension);

    let response = reqwest::get(target).await?;
    assert!(response.status().is_success());

    let content = response.bytes().await?;

    let path = Path::new("dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(&content)?;
    Ok(())
}
