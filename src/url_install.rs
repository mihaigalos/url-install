use crate::slicer::Slicer;
use crate::traits::{Decompressor, Downloader};

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

use std::{fs, path::Path};

use is_executable::is_executable;

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

        let executable = &UrlInstall::get_executable(archive_without_extension);

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

    fn get_executable(archive_without_extension: &str) -> Option<String> {
        let mut executable: String = "".to_string();
        match UrlInstall::get_target(archive_without_extension) {
            Some(x) => {
                let target_type = fs::metadata(x).unwrap();
                if target_type.is_file() {
                    UrlInstall::ensure_executable_permissions(x).ok()?;
                    executable = x.to_string();
                } else {
                    println!("Looking for executable files in: {}.", x);
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
            println!("Found executable: {:?}", executable);
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
}
