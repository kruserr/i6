# Contributing

Welcome, and thanks for your interest in contributing!

Before submitting a pull request please make sure that your code follows the style guide.

## Style Guide

- **Commits** follow the ["Conventional Commits" specification](https://www.conventionalcommits.org/en/v1.0.0/) 
- **Code** is formatted via nightly `cargo +nightly fmt --all` and linted via stable `cargo clippy --all-targets --all-features -- -Dwarnings`
- **Unsafe** is used when appropriate [Rustonomicon](https://doc.rust-lang.org/nomicon/)

## Getting Started
Clone the repo and build, then test, format and lint your changes
```bash
git clone https://github.com/kruserr/i6.git
cd i6
cargo build

# Make your changes

cargo test
cargo +nightly fmt --all
cargo clippy --all-targets --all-features -- -Dwarnings
```

### VS Code Extensions
If using VS Code as your IDE, install the following extensions for a reasonably good developer experience:
- `rust-lang.rust-analyzer`
- `vadimcn.vscode-lldb`
