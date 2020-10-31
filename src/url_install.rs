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
        let temporary_folder = UrlInstall::temporary_folder();
        let archive_file = &(temporary_folder.clone() + Slicer::target_with_extension(from_url));

        self.downloader.get(from_url, archive_file)?;
        self.decompressor.run(archive_file)?;
        std::fs::remove_file(archive_file).unwrap();

        let archive_without_extension =
            &(temporary_folder.clone() + Slicer::target_without_extension(archive_file));

        // self.ensure_executable_permissions(target)?;

        Ok(())
    }

    #[cfg(any(target_os = "linux", target_os = "mac"))]
    fn ensure_executable_permissions(file: &str) -> std::io::Result<()> {
        fs::set_permissions(file, fs::Permissions::from_mode(0o755))?;
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn ensure_executable_permissions(file: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn temporary_folder() -> String {
        fs::create_dir("tmp").unwrap();
        return "tmp/".to_string();
    }

    fn get_target(file: &str) -> Option<&str> {
        let path = file;
        let mut i = path.len();

        while i > 0 {
            if Path::new(&path[..i]).exists() {
                return Some(&path[..i]);
            } else {
                if path.to_string().chars().nth(i - 1).unwrap() == '/' {
                    return None;
                } else {
                }
            }
            i -= 1;
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_target() {
        let expected = "test/test_decompression.zip";

        let is = UrlInstall::get_target("test/test_decompression.zipAAAA").unwrap();

        assert_eq!(is, expected);
    }
}
