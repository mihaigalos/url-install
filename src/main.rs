use error_chain::error_chain;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = "https://github.com/twbs/bootstrap/archive/v4.0.0.zip";
    let response = reqwest::get(target).await?;
    assert!(response.status().is_success());

    let content = response.bytes().await?;

    let path = Path::new("./download.zip");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(&content)?;
    Ok(())
}
