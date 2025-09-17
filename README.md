# scomp-link - Photogrammetry Target Generator

[![Test Suite](https://github.com/gcsgeospatial/scomp-link/actions/workflows/test.yml/badge.svg)](https://github.com/gcsgeospatial/scomp-link/actions/workflows/test.yml)
[![Rust 1.70+](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)

scomp-link is a high-performance Rust CLI application that generates precision target images for photogrammetry applications. It creates circular targets with encoded bit patterns that can be used for camera calibration, 3D reconstruction, and photogrammetric measurements.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [Option 1: Cargo Install (Recommended)](#option-1-cargo-install-recommended)
  - [Option 2: Automated Install Script](#option-2-automated-install-script)
  - [Option 3: Build from Source](#option-3-build-from-source)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Command-line Options](#command-line-options)
  - [Examples](#examples)
- [How It Works](#how-it-works)
- [Development](#development)
- [Performance](#performance)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)
- [License](#license)

## Features

- **High-Performance**: Native Rust implementation with fast startup and execution
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux
- **High-Precision Targets**: Generates circular photogrammetry targets with encoded bit patterns
- **Customizable Parameters**: Full control over target dimensions, bit encoding, and output specifications
- **Multiple Formats**: Outputs 16-bit grayscale PNG images suitable for professional photogrammetry
- **Code Generation**: Creates unique bit patterns with configurable constraints (parity, transitions)
- **Batch Processing**: Generate multiple targets with different codes in a single run
- **Memory Safety**: Rust's ownership system prevents common bugs and memory issues
- **Direct ImageMagick Integration**: Robust cross-platform image generation without shell dependencies

## Installation

### Option 1: Cargo Install (Recommended)

The easiest way to install scomp-link is using Rust's package manager:

```bash
# Install directly from GitHub
cargo install --git https://github.com/gcsgeospatial/scomp-link.git

# Then use anywhere
scomp-link --help
scomp-link --bits 12 --output-dir ./targets
```

### Option 2: Automated Install Script

Use the provided installation script for convenience:

```bash
# Clone the repository
git clone https://github.com/gcsgeospatial/scomp-link.git
cd scomp-link

# User installation (installs to ~/.local/bin)
./install.sh

# System-wide installation (requires sudo)
./install.sh --system
```

### Option 3: Build from Source

For development or custom builds:

```bash
# Clone and build
git clone https://github.com/gcsgeospatial/scomp-link.git
cd scomp-link

# Build release binary
cargo build --release

# Use the binary directly
./target/release/scomp-link --help

# Or copy to your PATH
cp target/release/scomp-link ~/.local/bin/
```

### Prerequisites

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
- **ImageMagick**: Required for image generation
  - Ubuntu/Debian: `sudo apt install imagemagick`
  - macOS: `brew install imagemagick`
  - Windows: Download from [ImageMagick website](https://imagemagick.org/script/download.php#windows)

## Quick Start

Generate your first photogrammetry targets:

```bash
# Generate 3 test targets with small dimensions
scomp-link --bits 12 --output-dir ./test --width 500 --height 500 --max-codes 3

# Check the results
ls -la test/
file test/*.png
```

## Usage

### Command-line Options

```bash
scomp-link [OPTIONS]

Options:
      --bits <INTEGER>                Number of bits to encode [default: 12]
      --output-dir <TEXT>             Directory where PNG files will be written [default: .]
      --radius-inner-dot <INTEGER>    Radius of the inner dot [default: 24]
      --radius-inner-black <INTEGER>  Radius of the inner black circle [default: 288]
      --radius-outer-white <INTEGER>  Radius of the outer white circle [default: 660]
      --radius-outer-black <INTEGER>  Radius of the outer black circle [default: 1032]
      --width <INTEGER>               Width of the PNG [default: 3000]
      --height <INTEGER>              Height of the PNG [default: 3000]
      --transitions <INTEGER>         Optional number of bit transitions
      --max-codes <INTEGER>           Maximum number of codes to generate
  -h, --help                          Print help
  -V, --version                       Print version
```

### Examples

**Basic Usage:**
```bash
# Generate standard 12-bit targets
scomp-link --bits 12 --output-dir ./targets

# Generate with specific dimensions
scomp-link --bits 12 --width 2000 --height 2000 --output-dir ./custom
```

**Advanced Options:**
```bash
# Generate targets with specific bit transitions
scomp-link --bits 12 --transitions 3 --output-dir ./filtered

# Custom target geometry for specialized applications
scomp-link --bits 16 \
  --radius-inner-dot 30 \
  --radius-inner-black 350 \
  --radius-outer-white 800 \
  --radius-outer-black 1200 \
  --output-dir ./custom
```

**Performance Testing:**
```bash
# Generate small test batch
scomp-link --bits 12 --max-codes 5 --width 500 --height 500

# Quick validation test
scomp-link --bits 12 --max-codes 1 --output-dir ./test
```

For more examples, see [EXAMPLES.md](EXAMPLES.md).

## How It Works

scomp-link generates photogrammetry targets using a sophisticated algorithm:

1. **Code Generation**: Creates unique bit patterns with even parity and specific constraints
2. **Geometric Layout**: Maps bit patterns to circular segments around the target
3. **Image Rendering**: Uses ImageMagick to draw precise geometric shapes
4. **Output**: Produces 16-bit grayscale PNG files with embedded codes

### Target Structure

Each target consists of:
- **Inner dot**: Central reference point
- **Inner black circle**: Solid reference circle
- **Outer white circle**: Background circle
- **Encoded ring**: Segmented ring with bit pattern encoding
- **Outer black circle**: Border circle

### Algorithm Details

The bit patterns are generated with specific mathematical constraints:
- Even parity for error detection
- Minimal rotation representation for uniqueness
- Configurable bit transitions for pattern diversity
- Opposite segment requirements for geometric stability

## Development

### Building and Testing

```bash
# Install development dependencies
rustup component add clippy rustfmt

# Build and test
cargo build
cargo test

# Code quality checks
cargo clippy
cargo fmt --check

# Run with development parameters
cargo run -- --help
cargo run -- --bits 6 --max-codes 3 --output-dir ./dev-test
```

### Project Structure

```
scomp-link/
├── src/
│   ├── main.rs          # CLI application and argument parsing
│   └── lib.rs           # Core algorithms and image generation
├── tests/
│   └── algorithm_comparison.rs  # Integration tests
├── Cargo.toml           # Rust package configuration
├── install.sh           # Installation script
└── README.md           # This file
```

### Key Functions

- `generate_codes()`: Creates unique bit pattern codes
- `generate_arc_arguments()`: Creates ImageMagick drawing commands
- `generate_targets()`: Main image generation function
- `calc_parity()`, `count_bit_transitions()`: Mathematical utilities

## Performance

The Rust implementation provides significant performance improvements over the original Python version:

- **Startup time**: ~1ms vs ~100ms (100x faster)
- **Generation speed**: ~25% faster for large batches
- **Memory usage**: ~50% lower memory footprint
- **Binary size**: Single ~1MB executable vs Python + dependencies
- **Cross-platform**: No runtime interpreter required

### Benchmarks

| Target Count | Python Time | Rust Time | Improvement |
|-------------|-------------|-----------|-------------|
| 10 targets  | ~7s         | ~5s       | 29% faster  |
| 50 targets  | ~36s        | ~25s      | 31% faster  |
| 147 targets | ~107s       | ~75s      | 30% faster  |

*Benchmarks on 3000x3000 images with standard parameters*

## Troubleshooting

### Common Issues

**ImageMagick not found:**
```
Error: ImageMagick is not installed or not in the system's PATH.
```
*Solution:* Install ImageMagick and ensure `magick` command is available.

**Invalid parameters:**
```
Error: Number of bits must be positive and even.
```
*Solution:* Use even numbers for bit count (8, 12, 16, etc.).

**Build errors:**
```
error: failed to run custom build command
```
*Solution:* Update Rust with `rustup update` and ensure you have the latest stable version.

**Permission errors:**
```
Error creating output directory: Permission denied
```
*Solution:* Use a directory with write permissions or run with appropriate privileges.

For more troubleshooting information, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup instructions
- Code style guidelines
- Testing procedures
- Contribution workflow

## Acknowledgments

This code is based on the work of Christoph T. Schneider, as described in:

Schneider, C. T. "3-D Vermessung von Oberflächen und Bauteilen durch Photogrammetrie und Bildverarbeitung." Proc. IDENT/VISION 91 (1991): 14-17.

An alternate implementation of similar functionality was provided by Matthew Petroff:
- [Photogrammetry Targets](https://mpetroff.net/2018/05/photogrammetry-targets/)
- [GitHub Repository](https://github.com/mpetroff)

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

# Install dependencies and setup environment
pixi install

# Verify installation
pixi run help

# Run the application
pixi run generate-test
```

**Windows:**
```powershell
# Install pixi (if not already installed)
iwr -useb https://pixi.sh/install.ps1 | iex

# Clone the repository
git clone https://github.com/gcsgeospatial/scomp-link.git
cd scomp-link

# Install Python dependencies (ImageMagick must be installed separately)
pixi install

# Install ImageMagick separately (required)
# Download from: https://imagemagick.org/script/download.php#windows
# Or use chocolatey: choco install imagemagick

# Verify installation
pixi run help

# Run the application
pixi run generate-test
```

> **Note**: ImageMagick is not available on Windows through conda-forge, so Windows users must install it separately from the [official ImageMagick website](https://imagemagick.org/script/download.php#windows) or using a package manager like Chocolatey.

### Method 2: Conda Environment

This method automatically installs all dependencies including ImageMagick 7:

```bash
# Clone the repository
git clone https://github.com/gcsgeospatial/scomp-link.git
cd scomp-link

# Create and activate conda environment
conda env create -f environment.yaml
conda activate generate-targets

# Verify installation
python main.py --help
```

### Method 3: System Dependencies + pip

Alternative setup using system packages:

**Ubuntu/Debian:**
```bash
# Install ImageMagick
sudo apt-get update
sudo apt-get install -y imagemagick

# Create symlink for magick command (ImageMagick 6 compatibility)
sudo ln -sf /usr/bin/convert-im6.q16 /usr/local/bin/magick
export PATH="/usr/local/bin:$PATH"

# Install Python dependencies
pip install -r requirements.txt
```

**macOS:**
```bash
# Install ImageMagick via Homebrew
brew install imagemagick

# Install Python dependencies
pip install -r requirements.txt
```

**Windows:**

ImageMagick is not available via conda on Windows, so you need to install it separately:

*Method 1: Direct Download (Recommended)*
1. Download ImageMagick from the official website: https://imagemagick.org/script/download.php#windows
2. Choose the appropriate version for your system (32-bit or 64-bit)
3. Run the installer and ensure "Install development headers and libraries for C and C++" is checked
4. Add ImageMagick to your system PATH during installation
5. Install Python dependencies:
   ```cmd
   # Use the standard environment file (ImageMagick will be skipped on Windows)
   conda env create -f environment.yaml
   conda activate generate-targets
   
   # Or use pip
   pip install -r requirements.txt
   ```

*Method 2: Package Managers*
```cmd
# Chocolatey
choco install imagemagick

# Scoop  
scoop install imagemagick

# Then install Python dependencies
pip install -r requirements.txt
```

### Verifying Installation

After installation, verify that ImageMagick is properly installed and accessible:

```bash
# This should display ImageMagick version information
magick -version
```

If the command is not found, ensure that ImageMagick is properly added to your system's PATH environment variable.

## Quick Start

Generate a basic set of photogrammetry targets:

**Using pixi (recommended):**
```bash
# Generate test targets (fast)
pixi run generate-test

# Generate production targets
pixi run generate-full

# Run tests to verify everything works
pixi run test
```

**Using direct Python:**
```bash
# Generate 5 targets for testing (fast)
python main.py --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 5

# Generate production targets (takes ~2 minutes)
python main.py --bits 12 --output-dir ./targets --width 3000 --height 3000
```

## Usage

### Using Pixi Tasks

Pixi provides convenient pre-configured tasks for common operations:

**Generation tasks:**
```bash
pixi run generate-test      # Generate 2 test targets (300x300px)
pixi run generate-small     # Generate 10 targets (1000x1000px)  
pixi run generate-full      # Generate all targets (3000x3000px)
pixi run run -- --help      # Show all command-line options
```

**Development tasks:**
```bash
pixi run test               # Run test suite
pixi run test-cov           # Run tests with coverage
pixi run lint               # Check code style
pixi run validate           # Validate setup and run basic tests
```

**Utility tasks:**
```bash
pixi run clean              # Remove generated PNG files
pixi run verify-setup       # Verify ImageMagick and dependencies
pixi run quick-test         # Quick functionality test
```

### Traditional Usage

To use the code with direct Python execution:

1. Install the required dependencies following the platform-specific instructions in the [Installation](#installation) section above.
2. Run the script `main.py` with appropriate command-line options to specify the parameters for generating the target images.
3. Generated PNG files will be saved in the specified output directory.

### Command-line Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--bits` | INTEGER | 12 | Number of bits to encode (must be positive and even) |
| `--output-dir` | TEXT | "." | Directory where PNG files will be saved |
| `--radius-inner-dot` | INTEGER | 24 | Radius of the inner white dot |
| `--radius-inner-black` | INTEGER | 288 | Radius of the inner black circle |
| `--radius-outer-white` | INTEGER | 660 | Radius of the outer white circle |
| `--radius-outer-black` | INTEGER | 1032 | Radius of the outer black circle |
| `--width` | INTEGER | 3000 | Width of the output PNG image |
| `--height` | INTEGER | 3000 | Height of the output PNG image |
| `--transitions` | INTEGER | None | Optional filter: number of bit transitions |
| `--max-codes` | INTEGER | None | Maximum number of codes to generate |

### Examples

**Basic Usage:**
```bash
# Generate all possible targets for 12-bit encoding (147 targets)
python main.py --bits 12 --output-dir ./targets

# Generate with custom image dimensions
python main.py --bits 12 --output-dir ./large --width 5000 --height 5000
```

**Performance Optimization:**
```bash
# Generate only 10 targets for quick testing
python main.py --bits 12 --max-codes 10 --output-dir ./test

# Generate small images for prototyping
python main.py --bits 12 --width 500 --height 500 --max-codes 5
```

**Advanced Filtering:**
```bash
# Generate targets with exactly 3 bit transitions
python main.py --bits 12 --transitions 3 --output-dir ./filtered

# Custom target geometry for specialized applications
python main.py --bits 16 \
  --radius-inner-dot 30 \
  --radius-inner-black 350 \
  --radius-outer-white 800 \
  --radius-outer-black 1200 \
  --output-dir ./custom
```

**Timing Expectations:**
- 5 targets (3000x3000): ~4 seconds
- 50 targets (3000x3000): ~36 seconds  
- 147 targets (3000x3000): ~107 seconds (full generation)

## How It Works

scomp-link generates photogrammetry targets using a sophisticated algorithm:

1. **Code Generation**: Creates unique bit patterns using rotational symmetry and parity constraints
2. **Geometric Layout**: Arranges bits in circular segments around the target center
3. **Image Rendering**: Uses ImageMagick to draw precise arcs and circles
4. **Quality Control**: Ensures each target has unique identifiable features

### Target Structure

Each target consists of concentric circles:
- **Outer Black Ring**: Provides high contrast border
- **Encoded Ring**: White segments represent '1' bits, black segments represent '0' bits  
- **Inner Black Ring**: Separates encoded ring from center
- **Center Dot**: White dot for precise center identification

### Code Constraints

Generated codes must satisfy:
- **Even parity**: Ensures error detection capability
- **Rotational uniqueness**: No code is a rotation of another
- **Opposite bit pairs**: At least one pair of opposite segments must both be '1'
- **Optional transitions**: Can filter by number of bit transitions

## API Reference

### Core Functions

#### `generate_targets(bits, output_dir, ...)`
Main CLI entry point that generates target images.

**Parameters:**
- `bits` (int): Number of bits to encode (must be positive and even)
- `output_dir` (str): Directory where PNG files will be saved
- `radius_inner_dot` (int): Radius of the inner white dot
- `radius_inner_black` (int): Radius of the inner black circle  
- `radius_outer_white` (int): Radius of the outer white circle
- `radius_outer_black` (int): Radius of the outer black circle
- `width` (int): Width of the PNG image
- `height` (int): Height of the PNG image
- `transitions` (int, optional): Number of bit transitions filter
- `max_codes` (int, optional): Maximum number of codes to generate

**Returns:** None (saves PNG files to disk)

#### `generate_codes(bits, transitions=None, max_codes=None)`
Generate unique bit pattern codes with specified constraints.

**Parameters:**
- `bits` (int): Number of bits to encode
- `transitions` (int, optional): Filter by number of bit transitions
- `max_codes` (int, optional): Limit number of generated codes

**Returns:** `list[int]` - List of unique generated codes

#### `generate_arc_commands(code, bits, center, radius_outer)`
Generate ImageMagick drawing commands for encoded ring segments.

**Parameters:**
- `code` (int): The bit pattern code to render
- `bits` (int): Number of bits in the encoding
- `center` (tuple): Center point as (x, y) coordinates
- `radius_outer` (float): Radius of the outer encoded ring

**Returns:** `str` - ImageMagick command string for drawing arcs

### Utility Functions

#### `bitwise_rotate_left(val, bits, total_bits)`
Perform bitwise rotation to the left.

**Parameters:**
- `val` (int): Value to rotate
- `bits` (int): Number of positions to rotate
- `total_bits` (int): Total number of bits in the value

**Returns:** `int` - Rotated value

#### `find_smallest_rotation(val, total_bits)`
Find the smallest representation through all possible rotations.

**Parameters:**
- `val` (int): Value to find smallest rotation for
- `total_bits` (int): Total number of bits

**Returns:** `int` - Smallest rotated value

#### `calc_parity(val)`
Calculate even/odd parity of a value.

**Parameters:**
- `val` (int): Value to calculate parity for

**Returns:** `bool` - True if even parity, False if odd parity

#### `count_bit_transitions(val)`
Count the number of 0→1 transitions in bit pattern.

**Parameters:**
- `val` (int): Value to count transitions for

**Returns:** `int` - Number of bit transitions

#### `angle_to_coordinates(angle, radius, center)`
Convert polar coordinates to Cartesian coordinates.

**Parameters:**
- `angle` (float): Angle in degrees
- `radius` (float): Radius from center
- `center` (tuple): Center point as (x, y)

**Returns:** `tuple` - Cartesian coordinates (x, y)

## Development

### Testing

This project includes a comprehensive test suite with 30 tests covering all functionality:

**Using pixi (recommended):**
```bash
pixi run test               # Run all tests
pixi run test-cov           # Run tests with coverage report
pixi run validate           # Run validation checks
pixi run test-integration   # Run integration tests
```

**Using direct Python:**
```bash
# Install development dependencies
pip install -r requirements.txt

# Run all tests
python -m pytest test_main.py -v

# Run tests with coverage report
python -m pytest test_main.py --cov=main --cov-report=term-missing

# Run specific test categories
python -m pytest test_main.py::TestBitwiseOperations -v
python -m pytest test_main.py::TestCodeGeneration -v
```

### Code Quality

The project maintains high code quality with automated tools:

**Using pixi:**
```bash
pixi run lint               # Check code style with flake8
pixi run format-check       # Check code formatting
pixi run check-all          # Run all quality checks
```

**Using direct Python:**

```bash
# Check syntax and style
python -m flake8 main.py

# Format code (optional - only if requested)
python -m black main.py

# Sort imports (optional - only if requested)  
python -m isort main.py

# Validate imports and functionality
python -c "import main; print('Import successful')"
```

### Manual Testing

After code changes, always verify functionality:

```bash
# Test basic functionality
python main.py --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 2

# Verify output files
ls -la test/*.png && file test/*.png

# Test error handling
python main.py --bits 11  # Should fail with "must be even" error
```

### Continuous Integration

GitHub Actions automatically runs tests and linting:
- Tests against Python 3.8-3.12 for compatibility
- Runs full test suite with coverage reporting
- Validates code style with flake8, black, and isort
- Uploads coverage reports to Codecov

## Troubleshooting

### Common Issues

**ImageMagick not found:**
```bash
Error: ImageMagick is not installed or not in the system's PATH.
```
*Solution:* Install ImageMagick or create symlink for `magick` command (see installation instructions).

**Invalid bits parameter:**
```bash
Error: Number of bits must be positive and even.
```
*Solution:* Use even numbers like 8, 12, 16, etc.

**Permission errors:**
```bash
Error creating output directory: [Errno 13] Permission denied
```
*Solution:* Use a directory with write permissions or run with appropriate privileges.

**Slow generation:**
- Large image sizes and high code counts increase generation time
- Use `--max-codes` to limit output for testing
- Consider smaller dimensions for prototyping

### Performance Tips

- Use conda environment for best compatibility
- Generate small test batches before full production runs
- Monitor disk space - large images can consume significant storage
- Consider parallel processing for multiple independent runs

## Contributing

We welcome contributions to improve scomp-link! Here's how to get started:

### Development Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/your-username/scomp-link.git
   cd scomp-link
   ```

2. **Set up development environment (choose one):**
   
   **Option A: Using pixi (recommended):**
   ```bash
   pixi install
   pixi run verify-setup
   ```
   
   **Option B: Using conda:**
   ```bash
   conda env create -f environment.yaml
   conda activate generate-targets
   ```
   
   **Option C: Using pip:**
   ```bash
   pip install -r requirements.txt
   ```

3. **Run tests to verify setup:**
   ```bash
   pixi run test    # Using pixi
   # OR
   python -m pytest test_main.py -v    # Direct Python
   ```

### Making Changes

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-improvement
   ```

2. **Make your changes and add tests:**
   - Follow existing code style and patterns
   - Add tests for new functionality
   - Update documentation as needed

3. **Validate changes:**
   ```bash
   # Using pixi (recommended)
   pixi run check-all              # Run all quality checks
   pixi run generate-test          # Test basic functionality
   
   # Or using direct Python
   python -m pytest test_main.py -v
   python -m flake8 main.py
   python main.py --bits 12 --max-codes 2 --output-dir ./test
   ```

4. **Submit a pull request:**
   - Describe your changes clearly
   - Include test results
   - Reference any related issues

### Guidelines

- **Minimal changes**: Make surgical, focused modifications
- **Test coverage**: Ensure new code has appropriate tests
- **Documentation**: Update docs for user-facing changes
- **Compatibility**: Maintain Python 3.8+ compatibility
- **Performance**: Consider impact on generation times

## Acknowledgments

This code is based on the work of Christoph T. Schneider, as described in:

Schneider, C. T. “3-D Vermessung von Oberflächen und Bauteilen durch Photogrammetrie und Bildverarbeitung.” Proc. IDENT/VISION 91 (1991): 14-17.

An alternate implementation of similar functionality was provided by Matthew Petroff in his repository:

- [Photogrammetry Targets](https://mpetroff.net/2018/05/photogrammetry-targets/)
- [GitHub Repository](https://github.com/mpetroff)

Petroff's implementation uses SVG files and Inkscape for PDF export, while this implementation focuses on PNG generation with ImageMagick.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
