use crate::traits::Downloader;
use http::StatusCode;
use std::{fs::File, io::Write, path::Path};

pub struct BlockingDownloader;
impl Downloader for BlockingDownloader {
    fn get(&self, from_url: &str, to_file: &str) -> std::io::Result<()> {
        let response = self.download(from_url);
        assert!(response.status() == StatusCode::OK);

        self.write_file(to_file, response)?;
        Ok(())
    }
}

impl BlockingDownloader {
    fn download(&self, from_url: &str) -> reqwest::blocking::Response {
        let response = reqwest::blocking::get(from_url).unwrap();
        response
    }

    fn write_file(
        &self,
        to_file: &str,
        response: reqwest::blocking::Response,
    ) -> std::io::Result<()> {
        let content = response.bytes().unwrap();
        let target_with_extension = Path::new(to_file);

        File::create(&target_with_extension)
            .expect("Unable to create file")
            .write_all(&content)?;
        Ok(())
    }
}
