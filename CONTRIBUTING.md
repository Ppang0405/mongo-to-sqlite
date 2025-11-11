# Contributing to MongoDB to SQLite Migration Tool

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- MongoDB instance (for testing)
- Git

### Setting Up Development Environment

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/mongo-to-sqlite
   cd mongo-to-sqlite
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your MongoDB connection details
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

## Development Workflow

### 1. Create a Branch

Create a feature branch for your work:

```bash
git checkout -b feature/your-feature-name
```

Use conventional branch names:
- `feature/` for new features
- `fix/` for bug fixes
- `docs/` for documentation changes
- `refactor/` for code refactoring
- `test/` for adding tests

### 2. Make Your Changes

Follow these guidelines:

#### Code Style
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write clear, self-documenting code
- Add comments for complex logic

#### Testing
- Write unit tests for new functionality
- Ensure all tests pass: `cargo test`
- Add integration tests where appropriate

#### Documentation
- Update relevant documentation in `docs/`
- Add docstring comments to public functions
- Update README.md if adding new features

### 3. Commit Your Changes

Use conventional commit messages:

```
type(scope): description

[optional body]

[optional footer]
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
```bash
git commit -m "feat(schema): add support for MongoDB Time Series collections"
git commit -m "fix(converter): handle null values in nested documents"
git commit -m "docs(examples): add real-world migration scenarios"
```

### 4. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Reference to related issues
- Screenshots (if UI changes)
- Test results

## Code Guidelines

### Rust Code Style

```rust
/// Function documentation with description
///
/// # Arguments
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When does this function return an error
pub fn example_function(param1: &str, param2: usize) -> Result<String> {
    // Implementation
    Ok(String::new())
}
```

### Error Handling

- Use `Result<T>` for operations that can fail
- Use `anyhow::Result` for application-level errors
- Use custom error types for library code
- Provide clear error messages

```rust
use anyhow::{Result, Context};

pub async fn process_data(data: &[u8]) -> Result<ProcessedData> {
    let parsed = parse_data(data)
        .context("Failed to parse input data")?;
    
    Ok(parsed)
}
```

### Async Code

- Use `async/await` syntax
- Prefer `tokio::spawn` for concurrent tasks
- Use `futures::stream` for streaming data

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_async_function() {
        // Test async code
    }
}
```

## Project Structure

```
mongo-to-sqlite/
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI argument parsing
│   ├── error.rs          # Error types
│   ├── mongodb_client.rs # MongoDB operations
│   ├── libsql_client.rs  # LibSQL operations
│   ├── schema.rs         # Schema inference
│   ├── converter.rs      # Type conversion
│   └── migration.rs      # Migration orchestration
├── docs/                 # Documentation
├── examples/             # Example scripts (future)
├── tests/                # Integration tests (future)
└── Cargo.toml           # Dependencies
```

## Adding New Features

### Example: Adding a New Type Converter

1. **Update converter.rs**
   ```rust
   pub fn convert_new_type(bson: &Bson) -> SqlValue {
       // Implementation
   }
   ```

2. **Add tests**
   ```rust
   #[test]
   fn test_convert_new_type() {
       let bson = Bson::NewType(...);
       let result = convert_new_type(&bson);
       assert_eq!(result, expected);
   }
   ```

3. **Update documentation**
   - Add to `docs/typeMappings.md`
   - Update README.md if needed

4. **Create PR**

## Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests (requires MongoDB)
cargo test --test '*' -- --ignored
```

## Documentation

### Building Documentation

```bash
# Build Rust docs
cargo doc --no-deps --open

# Preview GitHub Pages locally
# (requires Jekyll)
cd docs
bundle exec jekyll serve
```

### Writing Documentation

- Use clear, concise language
- Include code examples
- Add real-world use cases
- Keep documentation up-to-date with code

## Pull Request Process

1. **Before Submitting**
   - [ ] Code compiles without warnings
   - [ ] All tests pass
   - [ ] Code is formatted (`cargo fmt`)
   - [ ] No clippy warnings (`cargo clippy`)
   - [ ] Documentation is updated
   - [ ] Commit messages follow conventions

2. **PR Description**
   - Describe what changes were made and why
   - Reference related issues
   - List any breaking changes
   - Include testing instructions

3. **Review Process**
   - Maintainers will review your PR
   - Address feedback and comments
   - Keep the PR updated with main branch

4. **Merging**
   - PRs require approval from maintainer
   - Squash and merge is preferred
   - Delete branch after merge

## Reporting Issues

### Bug Reports

Include:
- Clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version, etc.)
- Error messages and logs

### Feature Requests

Include:
- Clear description of the feature
- Use case and motivation
- Example usage
- Potential implementation approach

## Code of Conduct

### Our Standards

- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what's best for the community

### Unacceptable Behavior

- Harassment or discrimination
- Trolling or insulting comments
- Public or private harassment
- Publishing others' private information

## Questions?

- Open an issue for general questions
- Use discussions for open-ended topics
- Tag maintainers for urgent issues

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🎉

