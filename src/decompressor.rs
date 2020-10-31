use crate::traits::Decompressor;
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

use unzip::Unzipper;

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

pub struct ZipDecompressor;
impl Decompressor for ZipDecompressor {
    fn run(&self, file: &str) -> std::io::Result<()> {
        Unzipper::new(File::open(file).unwrap(), ".")
            .unzip()
            .unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targz_decompression_works() {
        let in_file = "test/test_decompression.tar.gz";
        let out_file = "test_decompression.tar.gz.txt";

        TarGzDecompressor {}.run(in_file).unwrap();
        assert!(std::path::Path::new(out_file).exists());
        std::fs::remove_file(out_file).unwrap();
    }

    #[test]
    fn zip_decompression_works() {
        let in_file = "test/test_decompression.zip";
        let out_file = "test_decompression.zip.txt";

        ZipDecompressor {}.run(in_file).unwrap();
        assert!(std::path::Path::new(out_file).exists());
        std::fs::remove_file(out_file).unwrap();
    }
}
