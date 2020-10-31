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
