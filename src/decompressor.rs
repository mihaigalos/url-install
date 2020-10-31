use crate::traits::Decompressor;
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub struct TarGzDecompressor;
impl Decompressor for TarGzDecompressor {
    fn run(&self, file: &str) -> std::io::Result<()> {
        let tar_gz = File::open(file)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(".")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targz_decompression_works() {
        let mut is_file_downloaded = false;
        let file = "dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz";
        BlockingDownloader{}.get("https://github.com/Byron/dua-cli/releases/download/v2.10.2/dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz", file).unwrap();
        if Path::new(file).exists() {
            is_file_downloaded = true;
        }
        fs::remove_file(file).unwrap();
        assert!(is_file_downloaded);
    }
}
