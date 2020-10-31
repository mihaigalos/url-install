use crate::slicer::Slicer;
use crate::traits::{Decompressor, Downloader};

pub struct UrlInstall {
    pub downloader: Box<dyn Downloader>,
    pub decompressor: Box<dyn Decompressor>,
}
impl UrlInstall {
    pub fn run(&self, from_url: &str) -> std::io::Result<()> {
        self.downloader
            .get(from_url, Slicer::target_with_extension(from_url))?;

        Ok(())
    }
}
