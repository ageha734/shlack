# Shlack

[![CI/CD Pipeline](https://github.com/ageha734/shlack/actions/workflows/ci.yml/badge.svg)](https://github.com/ageha734/shlack/actions/workflows/ci.yml)
[![Documentation](https://github.com/ageha734/shlack/actions/workflows/docs.yml/badge.svg)](https://ageha734.github.io/shlack/)

A command line utility to pipe text into Slack messages, written in Rust.

```bash
echo "anyone wanna grab lunch? :thumbsup:" | shlack random
```

```bash
gist < file.json | shlack luke
```

```bash
hub pull-request | shlack general
```

## Features

- 🚀 Fast and lightweight Slack messaging client
- 🔧 Easy to use command-line interface
- 🧪 Comprehensive test coverage
- 🤖 Automated CI/CD pipeline with GitHub Actions
- 📦 Cross-platform releases (Linux, Windows, macOS)

## Install

### From Release

Download the latest release for your platform from the [releases page](https://github.com/ageha734/shlack/releases).

### From Source

Set an environment variable `SLACK_TOKEN`. You can get a token from the [Slack OAuth test token page](https://api.slack.com/docs/oauth-test-tokens)

```bash
cargo build --release
```

```bash
echo hi | target/release/shlack
```

This sends a message to Slackbot.

## Usage

```bash
echo "Hello, World!" | shlack general
```

## Commands

```bash
-v --verbose    # Enable verbose output
-p --prepend    # Prepend text to message
-a --append     # Append text to message
```

## Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Building

```bash
cargo build --release
```

## CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

1. **Lint**: Code formatting and clippy checks
2. **Test**: Run all unit tests
3. **Build**: Cross-platform builds for Linux, Windows, and macOS
4. **Release**: Automatic tagging and release creation when pushing to main/master

The pipeline automatically creates releases with binaries for all supported platforms when the version in `Cargo.toml` is updated.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
