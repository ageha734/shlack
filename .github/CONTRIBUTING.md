# Contributing to Shlack

Thank you for your interest in contributing to Shlack! We welcome contributions from everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Submitting Changes](#submitting-changes)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment
4. Create a new branch for your changes
5. Make your changes
6. Test your changes
7. Submit a pull request

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- A Slack workspace and token for testing

### Installation

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/shlack.git
cd shlack

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run -- --help
```

### Environment Setup

1. Get a Slack token from the [Slack OAuth test token page](https://api.slack.com/docs/oauth-test-tokens)
2. Set the environment variable:

   ```bash
   export SLACK_TOKEN=your_token_here
   ```

## Making Changes

### Before You Start

1. Check if there's already an issue for what you want to work on
2. If not, create an issue to discuss the change
3. Wait for feedback before starting work on large changes

### Branch Naming

Use descriptive branch names:

- `feature/add-emoji-support`
- `fix/handle-network-errors`
- `docs/update-installation-guide`

### Commit Messages

Follow conventional commit format:

- `feat: add emoji support for messages`
- `fix: handle network timeout errors`
- `docs: update installation instructions`
- `test: add unit tests for message formatting`
- `refactor: simplify error handling logic`

## Submitting Changes

### Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the version numbers in Cargo.toml if applicable
3. Ensure all tests pass
4. Ensure your code follows the coding standards
5. Fill out the pull request template completely
6. Link any related issues

### Pull Request Guidelines

- Keep changes focused and atomic
- Include tests for new functionality
- Update documentation as needed
- Ensure CI passes
- Respond to review feedback promptly

## Coding Standards

### Rust Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### Code Quality

- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names
- Keep functions small and focused
- Handle errors appropriately

### Performance

- Avoid unnecessary allocations
- Use appropriate data structures
- Profile performance-critical code
- Consider memory usage

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Writing Tests

- Write unit tests for all new functionality
- Include integration tests for complex features
- Test error conditions and edge cases
- Use descriptive test names
- Keep tests simple and focused

### Test Guidelines

- Tests should be deterministic
- Avoid external dependencies in tests
- Use mocks for external services
- Test both success and failure cases

## Documentation

### Code Documentation

- Document all public APIs
- Use rustdoc comments (`///`)
- Include examples in documentation
- Keep documentation up to date

### User Documentation

- Update README.md for user-facing changes
- Add examples for new features
- Update installation instructions if needed
- Keep documentation clear and concise

## Release Process

Releases are automated through GitHub Actions:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Create a pull request
4. After merge, the CI will create a release automatically

## Getting Help

- Create an issue for bugs or feature requests
- Use the question template for usage questions
- Join discussions in existing issues
- Be patient and respectful

## Recognition

Contributors will be recognized in:

- GitHub contributors list
- Release notes for significant contributions
- Special thanks in documentation

Thank you for contributing to Shlack! 🚀
