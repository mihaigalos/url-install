pub trait Downloader {
    fn get(&self, from_url: &str, to_file: &str) -> std::io::Result<()>;
}

pub trait Decompressor {
    fn run(&self, file: &str) -> std::io::Result<()>;
}
