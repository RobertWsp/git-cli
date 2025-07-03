# Contributing to Git CLI with Emojis

Thank you for your interest in contributing! This document provides guidelines for contributing to this project.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/yourusername/git-cli.git
   cd git-cli
   ```
3. **Install dependencies**:
   ```bash
   cargo build
   ```

## 🏗️ Development Setup

### Prerequisites
- Rust 1.70 or higher
- Git 2.0 or higher
- A terminal that supports ANSI colors

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check
```

## 📝 Code Style

- Follow standard Rust conventions
- Use `rustfmt` for formatting
- Run `clippy` and fix all warnings
- Write comprehensive tests for new features
- Add documentation for public APIs

### Commit Messages
Use this tool itself for commits! But follow these guidelines:
- Use conventional commit format: `type(scope): description`
- Keep the first line under 50 characters
- Use present tense ("add feature" not "added feature")
- Reference issues when applicable

## 🧪 Testing

### Unit Tests
Place unit tests in the same file as the code being tested:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

### Integration Tests
Place integration tests in the `tests/` directory.

### Test Coverage
We aim for high test coverage. Run coverage with:
```bash
cargo tarpaulin --out html
```

## 📦 Adding Dependencies

When adding new dependencies:
1. Use the minimal feature set needed
2. Prefer well-maintained crates
3. Update `Cargo.toml` with appropriate version constraints
4. Document the reason for the dependency

## 🐛 Bug Reports

When filing bug reports, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Git version (`git --version`)
- Steps to reproduce
- Expected vs actual behavior
- Relevant log output (use `--debug` flag)

## ✨ Feature Requests

For feature requests:
1. Check existing issues first
2. Describe the use case
3. Explain why it would be beneficial
4. Consider if it fits the project's scope

## 🔄 Pull Request Process

1. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Make your changes** following the code style guidelines

3. **Add tests** for any new functionality

4. **Update documentation** if needed

5. **Run the full test suite**:
   ```bash
   # Run all tests (unit + integration + doctests)
   cargo test --verbose --all-features
   
   # Run linting with all features
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Check code formatting
   cargo fmt --all -- --check
   
   # Build release version
   cargo build --release --verbose
   
   # Test binary functionality
   ./target/release/git-cli --version
   ./target/release/git-cli --help
   ```

6. **Commit your changes** using this tool:
   ```bash
   git-cli
   ```

7. **Push to your fork**:
   ```bash
   git push origin feature/amazing-feature
   ```

8. **Open a Pull Request** with:
   - Clear title and description
   - Reference to related issues
   - Screenshots if UI changes
   - Test results

## 📋 Review Process

- All PRs require at least one review
- CI must pass (tests, linting, formatting)
- Maintain backwards compatibility when possible
- Update changelog for user-facing changes

## 🏷️ Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.x.0`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions will handle the release

## 💡 Development Tips

### Debugging
Use the `--debug` flag to enable detailed logging:
```bash
git-cli --debug
```

### Testing with Different Git States
Create test repositories with various states:
```bash
mkdir test-repo && cd test-repo
git init
echo "test" > file.txt
git add file.txt
# Test various scenarios
```

### Architecture Overview
```
src/
├── main.rs           # CLI entry point
├── lib.rs           # Library exports
├── config/          # Configuration management
├── git/             # Git operations
├── ui/              # User interface
├── emojis.rs        # Emoji definitions
├── errors.rs        # Error types
├── utils.rs         # Utility functions
└── validation.rs    # Validation logic
```

## 🤝 Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Keep discussions on topic

## 📞 Getting Help

- Open an issue for bugs or feature requests
- Check existing issues and documentation
- Ask questions in discussions

Thank you for contributing! 🎉
