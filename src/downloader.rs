use crate::traits::Downloader;
use http::StatusCode;
use std::{fs::File, io::Write, path::Path};

pub struct BlockingDownloader;
impl Downloader for BlockingDownloader {
    fn run(&self, full_url: &str, file: &str) -> std::io::Result<()> {
        let response = self.download(full_url);
        assert!(response.status() == StatusCode::OK);

        self.write_file(file, response)?;
        Ok(())
    }
}

impl BlockingDownloader {
    fn download(&self, full_url: &str) -> reqwest::blocking::Response {
        let response = reqwest::blocking::get(full_url).unwrap();
        response
    }

    fn write_file(&self, file: &str, response: reqwest::blocking::Response) -> std::io::Result<()> {
        let content = response.bytes().unwrap();
        let target_with_extension = Path::new(file);

        File::create(&target_with_extension)
            .expect("Unable to create file")
            .write_all(&content)?;
        Ok(())
    }
}
