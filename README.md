# url-install ![Rust](https://github.com/mihaigalos/url-install/workflows/Rust/badge.svg) [![license](https://img.shields.io/badge/license-GPLv3-brightgreen.svg)](LICENSE) [![LoC](https://tokei.rs/b1/github/mihaigalos/url-install)](https://github.com/Aaronepower/tokei)

Install packages from remote archives using just their url.
Supported formats : `*.tar.gz`, `*.zip`.

### Building

```bash
cargo build
```

### Using

```bash
sudo target/debug/url-install [url]
```

### Example Usage

```bash
sudo target/debug/url-install https://github.com/ogham/exa/releases/download/v0.9.0/exa-linux-x86_64-0.9.0.zip
```