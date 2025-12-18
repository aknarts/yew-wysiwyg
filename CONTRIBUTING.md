# Contributing to yew-wysiwyg

Thank you for your interest in contributing to yew-wysiwyg! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check existing issues to avoid duplicates
2. Collect relevant information (OS, Rust version, browser, etc.)
3. Create a minimal reproducible example if possible

When creating a bug report, include:
- Clear, descriptive title
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if applicable
- Environment details

### Suggesting Features

Feature suggestions are welcome! Please:
1. Check if the feature has already been suggested
2. Clearly describe the use case and benefits
3. Consider the scope and maintainability
4. Be open to discussion and feedback

### Pull Requests

1. **Fork the repository** and create your branch from `main`

2. **Set up your development environment**:
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install trunk for demo development
   cargo install trunk

   # Clone your fork
   git clone https://github.com/yourusername/yew-wysiwyg.git
   cd yew-wysiwyg
   ```

3. **Make your changes**:
   - Write clear, documented code
   - Follow existing code style
   - Add tests for new functionality
   - Update documentation as needed

4. **Test your changes**:
   ```bash
   # Run tests
   cargo test --workspace --all-features

   # Check formatting
   cargo fmt --all -- --check

   # Run clippy
   cargo clippy --workspace --all-features -- -D warnings

   # Test the demo
   cd yew-wysiwyg-demo
   trunk serve
   ```

5. **Commit your changes**:
   - Use clear, descriptive commit messages
   - Reference issue numbers if applicable
   - Keep commits focused and atomic

6. **Push to your fork** and submit a pull request

7. **Wait for review**:
   - Address any feedback
   - Keep your PR up to date with main
   - Be patient and responsive

## Development Guidelines

### Code Style

- Follow Rust naming conventions
- Use `rustfmt` for formatting
- Address all `clippy` warnings
- Write self-documenting code with clear names
- Add comments for complex logic

### Documentation

- Document all public APIs with doc comments
- Include examples in doc comments where helpful
- Update README.md for user-facing changes
- Update CHANGELOG.md following Keep a Changelog format

### Testing

- Write unit tests for new functionality
- Test edge cases and error conditions
- Ensure tests are deterministic
- Use descriptive test names

### Commit Messages

Follow conventional commits format:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(widgets): add image widget with lazy loading

fix(editor): prevent widget deletion during drag

docs(readme): update installation instructions
```

## Project Structure

```
yew-wysiwyg/
├── yew-wysiwyg/          # Core library
│   ├── src/
│   │   ├── core/         # Core traits and abstractions
│   │   ├── editor/       # Editor components
│   │   ├── widgets/      # Standard widget implementations
│   │   └── lib.rs
│   └── Cargo.toml
├── yew-wysiwyg-demo/     # Demo application
└── .github/workflows/    # CI/CD pipelines
```

## Adding New Widgets

To add a new standard widget:

1. Create the widget struct in appropriate file under `widgets/`
2. Implement the `Widget` trait
3. Add factory method
4. Register in `WidgetRegistry::with_standard_widgets()`
5. Add tests
6. Update documentation

Example:

```rust
#[derive(Default)]
pub struct MyWidget;

impl MyWidget {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for MyWidget {
    fn widget_type(&self) -> &'static str {
        "my.widget"
    }

    fn render(&self, props: &WidgetProps) -> Html {
        html! { <div>{ "My Widget" }</div> }
    }

    // ... implement other trait methods
}
```

## Release Process

Releases are managed by maintainers:

1. Update version in Cargo.toml files
2. Update CHANGELOG.md
3. Create and push a version tag (e.g., `v0.2.0`)
4. GitHub Actions will automatically:
   - Run tests
   - Publish to crates.io
   - Create GitHub release
   - Deploy updated demo

## Getting Help

- Open an issue for questions
- Check existing documentation
- Review closed issues for similar problems
- Reach out to maintainers for guidance

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- Git history

Thank you for contributing to yew-wysiwyg!
