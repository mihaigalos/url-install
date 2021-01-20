# url-install
![Rust](https://github.com/mihaigalos/url-install/workflows/Rust/badge.svg) [![license](https://img.shields.io/badge/license-GPLv3-brightgreen.svg)](LICENSE) [![LoC](https://tokei.rs/b1/github/mihaigalos/url-install)](https://github.com/Aaronepower/tokei)

Install packages from remote archives using just their url.
Supported formats : `*.tar.gz`, `*.zip`.

`sudo` required if writing to system paths.

### Building

```bash
cargo build # Result now in target/debug/url-install
```

### Using

```bash
[sudo] url-install [url] [install_to_path]
```

### Example Usage

```bash
url-install https://github.com/ogham/exa/releases/download/v0.9.0/exa-linux-x86_64-0.9.0.zip /tmp

sudo url-install https://github.com/ogham/exa/releases/download/v0.9.0/exa-linux-x86_64-0.9.0.zip /usr/bin
```
