# Contributing to open-lark

Thank you for your interest in contributing to open-lark! This guide will help you get started.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Submitting Changes](#submitting-changes)
- [Style Guide](#style-guide)
- [Testing](#testing)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please read and follow it in all your interactions with the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/open-lark.git
   cd open-lark
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/foxzool/open-lark.git
   ```

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Protocol Buffers compiler (`protoc`)

### Setup Instructions

1. **Install Rust**: Visit [rustup.rs](https://rustup.rs/)

2. **Install protoc**:
   ```bash
   # macOS
   brew install protobuf
   
   # Ubuntu/Debian
   sudo apt-get install protobuf-compiler
   
   # Windows
   # Download from https://github.com/protocolbuffers/protobuf/releases
   ```

3. **Install development tools**:
   ```bash
   # Install just for running tasks
   cargo install just
   
   # Install components
   rustup component add rustfmt clippy
   ```

4. **Set up environment**:
   ```bash
   cp .env-example .env
   # Edit .env with your API credentials for testing
   ```

5. **Verify setup**:
   ```bash
   cargo build --all-features
   cargo test
   just lint
   ```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-bitable-api`
- `fix/token-refresh-issue`
- `docs/improve-readme`
- `refactor/error-handling`

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

body (optional)

footer (optional)
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code formatting
- `refactor`: Code restructuring
- `test`: Test additions/modifications
- `chore`: Maintenance tasks

Examples:
```
feat(bitable): add record batch update API
fix(auth): resolve token refresh race condition
docs(readme): update installation instructions
```

## Submitting Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and commit them

3. **Add tests** for new functionality

4. **Update documentation** if needed

5. **Run the test suite**:
   ```bash
   cargo test --all-features
   just lint
   ```

6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request** on GitHub

## Style Guide

### Rust Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Documentation

- Use `///` for public API documentation
- Include examples in doc comments when helpful
- Update README.md for significant changes
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)

### API Design

- Follow existing patterns in the codebase
- Use builder patterns for complex configuration
- Provide comprehensive error types
- Use appropriate visibility modifiers
- Consider backwards compatibility

## Testing

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_test
```

### Writing Tests

- Write unit tests for new functions
- Add integration tests for API endpoints
- Test error conditions
- Use descriptive test names
- Include both positive and negative test cases

### Test Requirements

- All new code must have tests
- Tests should cover edge cases
- Mock external dependencies appropriately
- Ensure tests are deterministic

## API Documentation

### Adding New APIs

1. **Implement the API client methods**
2. **Add comprehensive tests**
3. **Create an example** in the `examples/` directory
4. **Update documentation**
5. **Add to the feature list** in README.md

### Example Structure

```
examples/
  api/
    service_name/
      v1/
        operation_name.rs
```

## Release Process

Releases are handled by maintainers:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create a release PR
4. Use automated release: `just release 0.x.y`
5. GitHub Actions handles the rest

## Getting Help

- **Discord**: Join our [Discord server](https://discord.gg/your-invite)
- **Issues**: Create an issue for bugs or questions
- **Discussions**: Use GitHub Discussions for broader topics

## Recognition

Contributors will be recognized in:
- CHANGELOG.md for their contributions
- GitHub contributors list
- Release notes for significant contributions

Thank you for contributing to open-lark! 🚀