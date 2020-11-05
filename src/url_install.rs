extern crate chrono;

use crate::decompressor::{TarGzDecompressor, ZipDecompressor};

#[cfg(test)]
use crate::downloader::BlockingDownloader;
use crate::slicer::Slicer;
use crate::traits::{Decompressor, Downloader};

use chrono::Local;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use std::{fs, path::Path};

use is_executable::is_executable;
use rand::Rng;

pub struct UrlInstall {
    pub downloader: Box<dyn Downloader>,
    pub decompressor: Box<dyn Decompressor>,
}
impl UrlInstall {
    pub fn run(&self, from_url: &str, to_folder: &str) -> std::io::Result<()> {
        let temporary_folder = UrlInstall::temporary_folder();
        let archive_file = &(temporary_folder.clone() + Slicer::target_with_extension(from_url));

        println!("Downloading {}", archive_file);

        self.downloader.get(from_url, archive_file)?;
        self.decompressor.run(archive_file)?;
        std::fs::remove_file(archive_file).unwrap();

        let archive_without_extension =
            &(temporary_folder.clone() + Slicer::target_without_extension(archive_file));

        let executable = &UrlInstall::get_executable(archive_without_extension).unwrap();

        println!("Installing {}", to_folder.to_string() + Slicer::target(archive_without_extension));
        std::fs::rename(
            executable,
            to_folder.to_string() + Slicer::target(archive_without_extension),
        )?;

        UrlInstall::remove_temporary_folder(&temporary_folder)?;

        Ok(())
    }

    pub fn get_decompressor(url: &str) -> Box<dyn Decompressor> {
        let decompressor: Box<dyn Decompressor> = if url.ends_with(".tar.gz") {
            Box::new(TarGzDecompressor {})
        } else if url.ends_with(".zip") {
            Box::new(ZipDecompressor {})
        } else {
            panic!("Unknown extension type.")
        };

        decompressor
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
        let mut rng = rand::thread_rng();
        let random_number: u16 = rng.gen();

        let date = Local::now();
        let path = "tmp_".to_string()
            + &date.format("%Y%m%d_%H%M%S").to_string()
            + &"_".to_string()
            + &random_number.to_string()
            + &"/".to_string();

        if !Path::new(&path).exists() {
            fs::create_dir(&path).unwrap();
        }
        return path;
    }

    fn remove_temporary_folder(folder: &str) -> std::io::Result<()> {
        fs::remove_dir_all(folder)?;
        Ok(())
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

    fn get_executable(archive_without_extension: &str) -> Option<String> {
        let mut executable: String = "".to_string();
        match UrlInstall::get_target(archive_without_extension) {
            Some(x) => {
                let target_type = fs::metadata(x).unwrap();
                if target_type.is_file() {
                    UrlInstall::ensure_executable_permissions(x).ok()?;
                    executable = x.to_string();
                } else {
                    for file in fs::read_dir(x).unwrap() {
                        let path = file.unwrap().path();
                        if is_executable(&path) {
                            executable = path.to_str().unwrap().to_string();
                            break;
                        }
                    }
                }
            }
            None => {
                println!(
                    "Cannot determine executable in: {}.",
                    archive_without_extension
                );
            }
        }
        if executable.len() > 0 {
            Some(executable)
        } else {
            println!("No executables found.");
            None
        }
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

    #[test]
    fn get_executable_when_typical() {
        let expected = "test/example_executable";

        let is = UrlInstall::get_executable("test/example_executable").unwrap();

        assert_eq!(is, expected);
    }
    #[test]
    fn get_executable_when_in_subfolder() {
        let expected = "test/subfolder/example_executable";

        let is = UrlInstall::get_executable("test/subfolder/example_executable").unwrap();

        assert_eq!(is, expected);
    }
    #[test]
    #[should_panic]
    fn get_executable_panics_when_no_executables_in_subfolder() {
        UrlInstall::get_executable("test/subfolder/no_executables/example_executable").unwrap();
    }

    #[test]
    fn run_when_typical_tar_gz_archive() {
        let from_url = "https://github.com/Byron/dua-cli/releases/download/v2.10.2/dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz";
        let url_install = UrlInstall {
            downloader: Box::new(BlockingDownloader {}),
            decompressor: UrlInstall::get_decompressor(from_url),
        };
        let destination_folder = &UrlInstall::temporary_folder();
        url_install.run(from_url, destination_folder).unwrap();
        UrlInstall::remove_temporary_folder(destination_folder).unwrap();
    }
    #[test]
    fn run_when_typical_zip_archive() {
        let from_url =
            "https://github.com/ogham/exa/releases/download/v0.9.0/exa-linux-x86_64-0.9.0.zip";
        let url_install = UrlInstall {
            downloader: Box::new(BlockingDownloader {}),
            decompressor: UrlInstall::get_decompressor(from_url),
        };

        let destination_folder = &UrlInstall::temporary_folder();
        url_install.run(from_url, &destination_folder).unwrap();

        UrlInstall::remove_temporary_folder(destination_folder).unwrap();
    }
}
