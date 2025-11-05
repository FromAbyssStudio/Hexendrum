# Contributing to Hexendrum 🦀

Thank you for your interest in contributing to Hexendrum! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project is committed to providing a welcoming and inclusive environment for all contributors. Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before participating.

## Getting Started

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git
- Basic knowledge of Rust and audio concepts

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Hexendrum.git
   cd Hexendrum
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/RogueFairyStudios/Hexendrum.git
   ```

## Development Setup

### Quick Setup

Run the development setup script:
```bash
make setup-dev
```

### Manual Setup

1. Install Rust tools:
   ```bash
   rustup component add rustfmt
   rustup component add clippy
   ```

2. Install system dependencies (Linux only):
   ```bash
   # Debian/Ubuntu:
   sudo apt-get install libasound2-dev pkg-config
   
   # Arch Linux/SteamOS:
   sudo pacman -S alsa-lib pkg-config
   ```
   **Note**: The `alsa-sys` crate (used by `rodio`) requires ALSA development libraries on Linux.

3. Create configuration directories:
   ```bash
   mkdir -p ~/.config/hexendrum
   mkdir -p ~/.config/hexendrum/playlists
   ```

4. Build the project:
   ```bash
   cargo build
   ```

## Contributing Guidelines

### Types of Contributions

We welcome various types of contributions:

- 🐛 **Bug Fixes**: Fix issues and improve stability
- **New Features**: Add new functionality
- **Documentation**: Improve docs and examples
- **Tests**: Add or improve test coverage
- **Infrastructure**: Improve build system and CI/CD
- **UI/UX**: Enhance the user interface
- 🌍 **Localization**: Add language support

### Issue Guidelines

Before submitting an issue:

1. **Search existing issues** to avoid duplicates
2. **Use clear titles** that describe the problem
3. **Provide detailed descriptions** with steps to reproduce
4. **Include system information** (OS, Rust version, etc.)
5. **Add labels** to categorize the issue

### Pull Request Guidelines

1. **Create a feature branch** from `develop`:
   ```bash
   git checkout develop
   git pull upstream develop
   git checkout -b feature/your-feature-name
   ```

2. **Make focused changes** - one feature per PR
3. **Write clear commit messages** following conventional commits
4. **Update documentation** for new features
5. **Add tests** for new functionality
6. **Ensure all tests pass** before submitting

### Commit Message Format

Use conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Examples:
- `feat(audio): add FLAC format support`
- `fix(gui): resolve playlist display issue`
- `docs(readme): update installation instructions`
- `test(library): add metadata parsing tests`

## Code Style

### Rust Conventions

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `rustfmt` for consistent formatting
- Run `cargo clippy` to catch common issues
- Prefer `anyhow` for error handling
- Use meaningful variable and function names

### Project Structure

- Keep modules focused and cohesive
- Use `pub` only when necessary
- Document public APIs with doc comments
- Follow the existing module organization

### Error Handling

- Use `Result<T, E>` for fallible operations
- Provide meaningful error messages
- Use `anyhow::Context` for additional context
- Log errors appropriately

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Writing Tests

- Test both success and failure cases
- Use descriptive test names
- Mock external dependencies when possible
- Test edge cases and error conditions
- Aim for high test coverage

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, "expected");
    }
}
```

## Submitting Changes

### Before Submitting

1. **Ensure code compiles** without warnings
2. **Run all tests** and ensure they pass
3. **Format code** with `cargo fmt`
4. **Check with clippy** using `cargo clippy`
5. **Update documentation** if needed

### Pull Request Process

1. **Push your branch** to your fork
2. **Create a pull request** against `develop`
3. **Fill out the PR template** completely
4. **Request review** from maintainers
5. **Address feedback** and make requested changes
6. **Squash commits** if requested

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Other (please describe)

## Testing
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes
```

## Release Process

### Versioning

We use [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Steps

1. **Update version** in `Cargo.toml`
2. **Update changelog** with new features/fixes
3. **Create release branch** from `main`
4. **Run full test suite** and CI checks
5. **Create GitHub release** with changelog
6. **Merge to main** and tag release

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: Join our community server (link TBD)
- **Email**: Contact maintainers directly

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- Project README
- Release notes
- Contributor hall of fame

Thank you for contributing to Hexendrum! 🦀
