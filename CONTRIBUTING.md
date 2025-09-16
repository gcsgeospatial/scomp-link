# Contributing to scomp-link

Thank you for your interest in contributing to scomp-link! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Development Guidelines](#development-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Guidelines](#documentation-guidelines)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

This project and everyone participating in it is governed by our commitment to creating an inclusive, respectful, and collaborative environment. By participating, you are expected to uphold these values:

- **Be respectful**: Treat all community members with respect and kindness
- **Be inclusive**: Welcome newcomers and help them get started
- **Be collaborative**: Work together constructively and share knowledge
- **Be patient**: Help others learn and grow at their own pace

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- Python 3.8 or later
- Git for version control
- Basic familiarity with photogrammetry concepts (helpful but not required)
- Understanding of command-line tools

### First Steps

1. **Explore the project**: Read the README and try out the examples
2. **Run the tests**: Ensure everything works in your environment
3. **Look at open issues**: Find something that interests you
4. **Join discussions**: Ask questions and share ideas

## Development Setup

### Method 1: Pixi (Recommended)

[Pixi](https://pixi.sh) provides the most streamlined development experience:

**Linux and macOS:**
```bash
# Install pixi (if not already installed)
curl -fsSL https://pixi.sh/install.sh | bash

# Fork and clone your fork
git clone https://github.com/your-username/scomp-link.git
cd scomp-link

# Install all dependencies and setup environment
pixi install

# Verify setup
pixi run verify-setup
pixi run test
```

**Windows:**
```powershell
# Install pixi (if not already installed)
iwr -useb https://pixi.sh/install.ps1 | iex

# Fork and clone your fork
git clone https://github.com/your-username/scomp-link.git
cd scomp-link

# Install Python dependencies
pixi install

# Install ImageMagick separately (required)
# Download from: https://imagemagick.org/script/download.php#windows
# Or use chocolatey: choco install imagemagick

# Verify setup (after ImageMagick installation)
pixi run verify-setup
pixi run test
```

> **Windows Note**: ImageMagick must be installed separately on Windows as it's not available through conda-forge.

### Method 2: Conda Environment

```bash
# Fork and clone your fork
git clone https://github.com/your-username/scomp-link.git
cd scomp-link

# Create conda environment
conda env create -f environment.yaml
conda activate generate-targets

# Verify setup
python -m pytest test_main.py -v
python main.py --help
```

### Method 3: Virtual Environment

```bash
# Clone repository
git clone https://github.com/your-username/scomp-link.git
cd scomp-link

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Install ImageMagick (platform-specific)
# Ubuntu/Debian: sudo apt-get install imagemagick
# macOS: brew install imagemagick

# Verify setup
python -m pytest test_main.py -v
```

### Development Tools

**Using pixi (all tools included automatically):**
```bash
pixi run lint          # Code style check
pixi run test          # Run tests
pixi run format-check  # Check code formatting
pixi run check-all     # Run all quality checks
```

**Using pip (manual installation):**

```bash
# Already included in requirements.txt
pip install flake8 black isort pytest pytest-cov

# Optional: additional tools
pip install mypy  # Type checking
pip install pre-commit  # Git hooks
```

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

1. **Bug Reports**: Help us identify and fix issues
2. **Feature Requests**: Suggest new functionality
3. **Code Contributions**: Implement features or fix bugs
4. **Documentation**: Improve docs, examples, or comments
5. **Testing**: Add tests or improve test coverage
6. **Performance**: Optimize algorithms or workflows

### Finding Work

Good places to start:

- Issues labeled `good first issue`
- Issues labeled `help wanted`
- Documentation improvements
- Test coverage gaps
- Performance optimizations

### Bug Reports

When reporting bugs, include:

```markdown
**Environment:**
- OS: [e.g., Ubuntu 20.04, macOS 12.0, Windows 10]
- Python version: [e.g., 3.9.7]
- ImageMagick version: [output of `magick --version`]

**Steps to reproduce:**
1. Command run: `python main.py ...`
2. Expected behavior: [what should happen]
3. Actual behavior: [what actually happened]
4. Error messages: [full error output]

**Additional context:**
- Files involved: [if relevant]
- System resources: [if performance-related]
```

### Feature Requests

When suggesting features:

1. **Describe the problem**: What limitation are you facing?
2. **Propose a solution**: How should it work?
3. **Consider alternatives**: Are there other approaches?
4. **Assess impact**: Who would benefit from this feature?

## Development Guidelines

### Code Style

We follow Python community standards:

```bash
# Check code style
python -m flake8 main.py

# Format code (run before committing)
python -m black main.py
python -m isort main.py

# Our configuration (in setup.cfg):
# - Line length: 127 characters
# - Import sorting: black-compatible
# - Ignore: E203, W503 (black compatibility)
```

### Coding Principles

1. **Minimal changes**: Make surgical, focused modifications
2. **Backward compatibility**: Don't break existing functionality
3. **Clear intent**: Code should be self-documenting
4. **Performance awareness**: Consider generation time impact
5. **Error handling**: Provide helpful error messages

### Function Design

Good function design patterns in this project:

```python
def my_function(required_param, optional_param=None):
    """
    Brief description of what the function does.
    
    Parameters:
    required_param (type): Description of parameter.
    optional_param (type, optional): Description. Default is None.
    
    Returns:
    type: Description of return value.
    
    Raises:
    ValueError: When parameter validation fails.
    """
    # Validate inputs first
    if required_param <= 0:
        raise ValueError("required_param must be positive")
    
    # Main logic
    result = do_computation(required_param)
    
    # Return with clear type
    return result
```

### Error Handling

Follow project patterns for error handling:

```python
# CLI error handling (user-friendly messages)
if not os.path.exists(output_dir):
    try:
        os.makedirs(output_dir)
    except OSError as e:
        click.echo(f"Error creating output directory: {e}")
        return

# Function error handling (raise informative exceptions)
if bits <= 0 or bits % 2 != 0:
    raise ValueError("Number of bits must be positive and even")
```

## Testing Guidelines

### Test Structure

Our test suite uses pytest with organized test classes:

```python
class TestMyFeature:
    """Test my new feature functionality."""
    
    def test_basic_functionality(self):
        """Test basic case with expected inputs."""
        result = my_function(valid_input)
        assert result == expected_output
    
    def test_edge_cases(self):
        """Test boundary conditions."""
        # Test minimum values
        result = my_function(1)
        assert result is not None
        
        # Test maximum reasonable values
        result = my_function(1000)
        assert result is not None
    
    def test_error_conditions(self):
        """Test error handling."""
        with pytest.raises(ValueError):
            my_function(-1)
```

### Writing Tests

When adding features, include tests for:

1. **Basic functionality**: Normal use cases
2. **Edge cases**: Boundary conditions, empty inputs
3. **Error cases**: Invalid inputs, error conditions
4. **Integration**: How it works with existing code

### Running Tests

```bash
# Run all tests
python -m pytest test_main.py -v

# Run specific test class
python -m pytest test_main.py::TestMyFeature -v

# Run with coverage
python -m pytest test_main.py --cov=main --cov-report=term-missing

# Run tests with timing
python -m pytest test_main.py -v --durations=10
```

### Performance Testing

For performance-critical changes:

```python
import time

def test_performance_regression():
    """Ensure new code doesn't significantly slow down generation."""
    start_time = time.time()
    codes = main.generate_codes(12, max_codes=10)
    duration = time.time() - start_time
    
    # Should complete within reasonable time
    assert duration < 1.0, f"Code generation took {duration:.2f}s, expected < 1.0s"
    assert len(codes) == 10
```

## Documentation Guidelines

### Documentation Types

1. **Code comments**: Explain complex logic
2. **Docstrings**: Document all public functions
3. **README updates**: Reflect new features
4. **Example updates**: Show new functionality

### Docstring Format

Use Google-style docstrings:

```python
def complex_function(param1, param2=None):
    """
    One-line summary of the function.
    
    Longer description if needed, explaining the purpose,
    behavior, and any important details.
    
    Args:
        param1 (int): Description of the first parameter.
        param2 (str, optional): Description of optional parameter.
            Defaults to None.
    
    Returns:
        list: Description of return value and its structure.
    
    Raises:
        ValueError: When param1 is negative.
        TypeError: When param2 is not a string.
    
    Example:
        >>> result = complex_function(5, "test")
        >>> len(result)
        5
    """
```

### Comment Guidelines

Good comments explain *why*, not *what*:

```python
# Good: Explains reasoning
# Use smallest rotation to ensure unique codes
code = find_smallest_rotation(code, bits)

# Bad: Describes what's obvious
# Assign the result to code variable
code = find_smallest_rotation(code, bits)
```

### Documentation Testing

Test documentation examples:

```python
def test_docstring_examples():
    """Verify that examples in docstrings actually work."""
    # Example from complex_function docstring
    result = complex_function(5, "test")
    assert len(result) == 5
```

## Pull Request Process

### Before Submitting

1. **Test thoroughly**: Run full test suite
2. **Check code style**: Use flake8, black, isort
3. **Update documentation**: Reflect any changes
4. **Verify functionality**: Test with real examples

```bash
# Pre-submission checklist
python -m pytest test_main.py -v                    # All tests pass
python -m flake8 main.py                            # No style issues
python main.py --bits 12 --max-codes 2 --output-dir ./test  # Basic functionality
```

### Pull Request Template

Structure your PR description:

```markdown
## Description
Brief summary of changes and motivation.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Other: ___________

## Testing
- [ ] All existing tests pass
- [ ] Added tests for new functionality
- [ ] Tested with real examples
- [ ] No performance regression

## Changes Made
- Specific change 1
- Specific change 2
- Updated documentation for X

## Breaking Changes
- None / List any breaking changes

## Additional Notes
Any additional information reviewers should know.
```

### Review Process

1. **Automated checks**: GitHub Actions will run tests
2. **Code review**: Maintainers will review your code
3. **Feedback**: Address any comments or suggestions
4. **Approval**: Once approved, we'll merge your PR

### After Merge

- Your contribution will be included in the next release
- Thank you! You're now a contributor to scomp-link

## Release Process

### Versioning

We use [Semantic Versioning](https://semver.org/):

- **Major** (X.0.0): Breaking changes
- **Minor** (0.X.0): New features, backward compatible
- **Patch** (0.0.X): Bug fixes, backward compatible

### Release Checklist

For maintainers preparing releases:

1. **Update version numbers**: In relevant files
2. **Update changelog**: Document all changes
3. **Run full test suite**: On multiple Python versions
4. **Create release tag**: With semantic version
5. **Publish release**: With release notes

### Changelog Format

Document changes clearly:

```markdown
## [1.2.0] - 2024-01-15

### Added
- New feature for custom bit pattern filtering
- Support for 16-bit encoding
- Performance optimization examples

### Changed
- Improved error messages for invalid parameters
- Updated documentation with more examples

### Fixed
- Bug in arc command generation for edge cases
- Memory usage optimization for large target sets

### Deprecated
- Old parameter names (still supported, will warn)
```

## Questions and Support

### Getting Help

- **Documentation**: Check README, EXAMPLES.md, and code comments
- **Issues**: Search existing issues for similar problems
- **Discussions**: Start a discussion for general questions
- **Contact**: Reach out to maintainers for complex issues

### Community Resources

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Pull Requests**: Code contributions and reviews

Thank you for contributing to scomp-link! Your efforts help make photogrammetry more accessible and powerful for everyone.