# Git CLI with Emojis 🎨

[![CI](https://github.com/yourusername/git-cli/workflows/CI/badge.svg)](https://github.com/yourusername/git-cli/actions)
[![Crates.io](https://img.shields.io/crates/v/git-cli-emoji.svg)](https://crates.io/crates/git-cli-emoji)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful and modern CLI tool for Git commits with emoji support, conventional commit validation, and intelligent workflow automation.

## ✨ Features

- 🎭 **Interactive emoji selection** with contextual suggestions
- 📋 **Conventional Commits** support and validation
- 🔧 **Configurable** via TOML configuration files
- 🚀 **Smart workflow automation** (staging, committing, pulling, pushing)
- 🎯 **Pre-commit hook integration** with automatic retry
- 📊 **Commit analytics** and recent history display
- 🌐 **Non-interactive mode** for CI/CD integration
- 🔍 **Intelligent file analysis** for emoji suggestions
- ⚡ **Async operations** for better performance

## 🚀 Installation

### Via Cargo
```bash
cargo install git-cli-emoji
```

### From Source
```bash
git clone https://github.com/yourusername/git-cli.git
cd git-cli
cargo install --path .
```

## 📖 Usage

### Basic Usage
```bash
# Interactive mode (default)
git-cli

# Non-interactive mode
git-cli --no-interactive --emoji "✨" --title "Add new feature" --body "Implement user authentication"

# Debug mode
git-cli --debug
```

### Command Line Options
```
Options:
  -d, --debug              Enable debug mode
      --no-interactive     Run in non-interactive mode
      --emoji <EMOJI>      Emoji to use for commit
      --title <TITLE>      Commit title
      --body <BODY>        Commit body/description
  -h, --help              Print help
  -V, --version           Print version
```

## ⚙️ Configuration

Git CLI creates a configuration file at `~/.config/git-cli/config.toml`:

```toml
[general]
default_emoji = "✨"
auto_push = false
confirm_before_push = true
debug = false

[commit]
enforce_conventional = true
max_title_length = 50
max_body_length = 72
auto_capitalize_title = true

[hooks]
run_pre_commit = true
auto_fix_lint = true
retry_on_failure = true
```

### Configuration Options

#### General Settings
- `default_emoji`: Default emoji when none is specified
- `auto_push`: Automatically push after successful commit
- `confirm_before_push`: Ask before pushing to remote
- `debug`: Enable debug logging

#### Commit Settings
- `enforce_conventional`: Validate conventional commit format
- `max_title_length`: Maximum characters in commit title
- `max_body_length`: Maximum characters per line in commit body
- `auto_capitalize_title`: Automatically capitalize first letter

#### Hook Settings
- `run_pre_commit`: Execute pre-commit hooks
- `auto_fix_lint`: Attempt to auto-fix linting issues
- `retry_on_failure`: Retry commit after hook failure

## 🎭 Emoji Suggestions

The tool intelligently suggests emojis based on file changes:

| File Type | Suggested Emoji | Description |
|-----------|----------------|-------------|
| `*.md`, `*.rst` | 📝 | Documentation |
| `*test*`, `*spec*` | ✅ | Tests |
| `*.css`, `*.scss` | 💄 | Styling |
| `*.json`, `*.toml` | 🔧 | Configuration |
| `Dockerfile` | 🐳 | Docker |
| `*security*`, `*auth*` | 🔒 | Security |
| `*performance*` | ⚡ | Performance |

## 📋 Conventional Commits

When `enforce_conventional` is enabled, commit titles must follow the format:

```
<type>(<scope>): <description>
```

Supported types:
- `feat`: New feature
- `fix`: Bug fix  
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes
- `build`: Build system changes
- `revert`: Reverting changes

## 🔄 Workflow

1. **Repository Validation**: Ensures you're in a Git repository
2. **Change Detection**: Scans for modified, added, deleted files
3. **File Staging**: Interactive or automatic file selection
4. **Emoji Selection**: Choose from curated emoji list with smart suggestions
5. **Commit Creation**: Input title and optional body with validation
6. **Pre-commit Hooks**: Automatic execution with retry on failure
7. **Remote Sync**: Fetch, pull, and push operations
8. **Summary Display**: Show recent commits and current branch

## 🧪 Development

### Prerequisites
- Rust 1.70+ 
- Git 2.0+

### Building
```bash
git clone https://github.com/yourusername/git-cli.git
cd git-cli
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out html

# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --all
```

### Project Structure
```
src/
├── main.rs           # Application entry point
├── config/           # Configuration management
├── git/              # Git operations
├── ui/               # User interface
├── emojis.rs         # Emoji definitions
├── errors.rs         # Error handling
├── utils.rs          # Utility functions
└── validation.rs     # Validation logic
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run linting (`cargo clippy`)
7. Format code (`cargo fmt`)
8. Commit using this tool! 😉
9. Push to your branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Conventional Commits](https://www.conventionalcommits.org/) specification
- [Gitmoji](https://gitmoji.dev/) for emoji inspiration
- [inquire](https://github.com/mikaelmello/inquire) for beautiful CLI interactions
- [clap](https://github.com/clap-rs/clap) for argument parsing

## 📊 Roadmap

- [ ] **v0.3.0**: Commit templates and advanced validation
- [ ] **v0.4.0**: Plugin system and integrations
- [ ] **v0.5.0**: AI-powered commit message suggestions
- [ ] **v1.0.0**: Stable release with full documentation

---

Made with ❤️ and ☕ by [RobertWsp](https://github.com/RobertWsp)
