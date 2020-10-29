use http::StatusCode;
use std::{fs::File, io::Write, path::Path};

pub struct Downloader;

impl Downloader {
    pub fn run(full_url: &str, file: &str) -> std::io::Result<()> {
        let response = Downloader::download(full_url);
        assert!(response.status() == StatusCode::OK);

        Downloader::write_file(file, response)?;
        Ok(())
    }

    fn download(full_url: &str) -> reqwest::blocking::Response {
        let response = reqwest::blocking::get(full_url).unwrap();
        response
    }

    fn write_file(file: &str, response: reqwest::blocking::Response) -> std::io::Result<()> {
        let content = response.bytes().unwrap();
        let target_with_extension = Path::new(file);

        File::create(&target_with_extension)
            .expect("Unable to create file")
            .write_all(&content)?;
        Ok(())
    }
}
