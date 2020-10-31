use crate::slicer::Slicer;
use crate::traits::{Decompressor, Downloader};

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use std::fs;

pub struct UrlInstall {
    pub downloader: Box<dyn Downloader>,
    pub decompressor: Box<dyn Decompressor>,
}
impl UrlInstall {
    pub fn run(&self, from_url: &str) -> std::io::Result<()> {
        let file = Slicer::target_with_extension(from_url);
        self.downloader.get(from_url, file)?;
        self.decompressor.run(file)?;

        self.ensure_executable_permissions(file)?;

        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "mac"))]
    fn ensure_executable_permissions(&self, file: &str) -> std::io::Result<()> {
        fs::set_permissions(file, fs::Permissions::from_mode(0o755))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn ensure_executable_permissions(&self, file: &str) -> std::io::Result<()> {
        Ok(())
    }
}
