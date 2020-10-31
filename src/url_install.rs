use crate::slicer::Slicer;
use crate::traits::{Decompressor, Downloader};

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use std::{fs, path::Path};

pub struct UrlInstall {
    pub downloader: Box<dyn Downloader>,
    pub decompressor: Box<dyn Decompressor>,
}
impl UrlInstall {
    pub fn run(&self, from_url: &str) -> std::io::Result<()> {
        let archive_file = &(self.temporary_folder() + Slicer::target_with_extension(from_url));
        println!("{}", archive_file);
        self.downloader.get(from_url, archive_file)?;
        self.decompressor.run(archive_file)?;
        // std::fs::remove_file(archive_file).unwrap();

        // let target = Slicer::target(archive_file);
        // if !Path::new(target).exists() {}

        // self.ensure_executable_permissions(target)?;

        Ok(())
    }

    fn temporary_folder(&self) -> String {
        fs::create_dir("tmp").unwrap();
        return "tmp/".to_string();
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
