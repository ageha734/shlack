# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- ARM64 support for all platforms (Linux, Windows, macOS)
- Automated install script (`install.sh`)
- Comprehensive OSS documentation and templates
- Issue templates for bug reports, feature requests, and questions
- Pull request template
- Contributing guidelines
- Code of Conduct
- Security policy
- Cross-platform CI/CD pipeline with GitHub Actions

### Changed

- Enhanced release process to include ARM64 binaries
- Updated documentation with installation instructions
- Improved CI/CD pipeline with multi-architecture builds

### Fixed

- Cross-compilation setup for ARM64 Linux builds

## [0.1.0] - 2024-XX-XX

### Added

- Initial release of Shlack
- Basic Slack messaging functionality
- Command-line interface with argparse
- Support for piping text to Slack channels
- Cross-platform builds (Linux, Windows, macOS x86_64)
- Basic CI/CD pipeline
- Unit tests
- Documentation
- Send messages to Slack channels via command line
- Pipe text input to Slack
- Verbose output option
- Prepend and append text options
- Environment variable configuration for Slack token

[Unreleased]: https://github.com/ageha734/shlack/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ageha734/shlack/releases/tag/v0.1.0
