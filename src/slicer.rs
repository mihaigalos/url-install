struct Slicer;
impl Slicer {
    fn target_with_extension(s: &str) -> &str {
        let pos_of_last_slash = s.rfind('/').unwrap();
        &s[pos_of_last_slash + 1..]
    }

    fn target(s: &str) -> &str {
        let target_with_extension = Slicer::target_with_extension(s);
        let pos_of_first_dash = target_with_extension.find('-').unwrap();
        &target_with_extension[..pos_of_first_dash]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_with_extension() {
        let is = Slicer::target_with_extension("https://github.com/Byron/dua-cli/releases/download/v2.10.2/dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz");
        let expected = "dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz";
        assert_eq!(is, expected);
    }
    #[test]
    fn test_target() {
        let is = Slicer::target("https://github.com/Byron/dua-cli/releases/download/v2.10.2/dua-v2.10.2-x86_64-unknown-linux-musl.tar.gz");
        let expected = "dua";
        assert_eq!(is, expected);
    }
}
