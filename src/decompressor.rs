use crate::traits::Decompressor;
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
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
        let mut is_file_present = false;
        let in_file = "test/test_decompression.tar.gz";
        let out_file = "test_decompression.txt";

        TarGzDecompressor {}.run(in_file).unwrap();
        if Path::new(out_file).exists() {
            is_file_present = true;
        }
        std::fs::remove_file(out_file).unwrap();
        assert!(is_file_present);
    }
}
