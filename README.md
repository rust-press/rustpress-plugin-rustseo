# RustPress Plugin: SEO

Search engine optimization tools for RustPress CMS.

[![CI](https://github.com/rust-press/rustpress-plugin-rustseo/actions/workflows/ci.yml/badge.svg)](https://github.com/rust-press/rustpress-plugin-rustseo/actions/workflows/ci.yml)
[![Release](https://github.com/rust-press/rustpress-plugin-rustseo/actions/workflows/release.yml/badge.svg)](https://github.com/rust-press/rustpress-plugin-rustseo/actions/workflows/release.yml)

## Features

- Meta tags
- Sitemaps
- Schema markup
- Analytics integration
- Keyword analysis

## Installation

### From GitHub Releases

1. Download the latest release ZIP from the [Releases](https://github.com/rust-press/rustpress-plugin-rustseo/releases) page
2. Upload via RustPress admin panel or extract to `plugins/` directory
3. Activate the plugin in the admin panel

### From Source

```bash
git clone https://github.com/rust-press/rustpress-plugin-rustseo.git
cd rustpress-plugin-rustseo
cargo build --release
```

## Configuration

Configure the plugin through the RustPress admin panel under **Settings > SEO**.

## Requirements

- RustPress 1.0.0 or later
- Rust 1.75+ (for building from source)

## Development

```bash
# Run tests
cargo test

# Build
cargo build --release

# Check code
cargo clippy
```

## Contributing

Contributions are welcome! Please read the [RustPress Contributing Guide](https://github.com/rust-press/rustpress/blob/main/CONTRIBUTING.md).

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [RustPress Core](https://github.com/rust-press/rustpress)
- [Documentation](https://rustpress.org/docs/plugins/rustseo)
- [Issue Tracker](https://github.com/rust-press/rustpress-plugin-rustseo/issues)
