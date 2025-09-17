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
