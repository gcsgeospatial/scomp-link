# scomp-link - Photogrammetry Target Generator

scomp-link is a Rust CLI application that generates target images with specified parameters for photogrammetry applications. It uses ImageMagick for drawing arcs and creating PNG files with circular targets encoded with specific bit patterns.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Method 1: Cargo Install (Recommended)
Install the application using Cargo (Rust package manager):
- `cargo install --git https://github.com/gcsgeospatial/scomp-link.git` -- installs scomp-link binary. Takes 30-60 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `scomp-link --help` -- verify installation and see available options

### Method 2: Build from Source
Build and run from the repository:
- `cargo build --release` -- builds the release binary. Takes 30-60 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `./target/release/scomp-link --help` -- verify installation and see options
- `cargo run --release -- --help` -- alternative way to run during development

### Method 3: Automated Install Script
Use the provided install script:
- `./install.sh` -- installs to ~/.local/bin (user installation)
- `./install.sh --system` -- installs system-wide (requires sudo)

## Running the Application

### Basic Usage
Run the application with default parameters:
- `scomp-link --bits 12 --output-dir ./output --width 3000 --height 3000`

### Common Parameters
- `--bits INTEGER`: Number of bits to encode (must be positive and even). Default: 12
- `--output-dir TEXT`: Directory where PNG files will be saved. Default: current directory
- `--width INTEGER`: Width of PNG image. Default: 3000
- `--height INTEGER`: Height of PNG image. Default: 3000
- `--max-codes INTEGER`: Maximum number of codes to generate. Default: all possible (147 for 12 bits)
- `--transitions INTEGER`: Optional number of bit transitions to filter codes
- `--radius-*`: Various radius parameters for circle sizing

### Timing Expectations
- **NEVER CANCEL** image generation commands. Generation times scale with number of codes and image size:
  - 10 codes at 3000x3000: ~5 seconds (faster than Python)
  - 50 codes at 3000x3000: ~25 seconds (faster than Python)
  - 100 codes at 3000x3000: ~50 seconds (faster than Python)
  - 147 codes at 3000x3000: ~75 seconds (faster than Python)
- Set timeout to 200+ seconds for full generation runs
- Use `--max-codes` parameter to limit generation for testing

### Example Commands
Test with small parameters:
```bash
scomp-link --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 5
```

Generate production targets:
```bash
scomp-link --bits 12 --output-dir ./targets --width 3000 --height 3000
```

Generate with specific transitions:
```bash
scomp-link --bits 12 --transitions 3 --output-dir ./filtered --width 3000 --height 3000 --max-codes 10
```

## Validation

### Always Test After Code Changes
Run these commands to verify functionality:
- `cargo check` -- check Rust syntax and types
- `cargo test` -- run all unit tests
- `scomp-link --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 2` -- basic functionality test
- `ls -la test/*.png && file test/*.png` -- verify PNG files are created correctly

### Manual Testing Scenarios
After making changes, always run through these scenarios:
- **Basic generation**: Create 2-3 images with small dimensions to verify core functionality
- **Parameter validation**: Test with odd number of bits (should fail with error message)
- **Different bit counts**: Test with bits=8, bits=12, bits=16 to verify different configurations
- **Large generation**: Test with max-codes=10 and full-size images (3000x3000) to verify performance

### Error Conditions to Test
- Missing ImageMagick: Should show "ImageMagick is not installed" error
- Invalid bits parameter: Should show "Number of bits must be positive and even" error
- Invalid output directory: Should create directory or show permission error

## Code Quality

### Linting and Formatting
The code follows Rust best practices:
- `cargo fmt` -- format code according to Rust standards
- `cargo clippy` -- lint code for common issues and improvements
- `cargo test` -- run all tests to ensure functionality
- **ALWAYS** run formatting and linting before committing changes

## Project Structure

### Core Files
- `src/main.rs` -- Main CLI application with argument parsing
- `src/lib.rs` -- Core library with image generation logic and algorithms
- `tests/` -- Integration tests comparing with Python implementation
- `Cargo.toml` -- Rust package configuration and dependencies
- `README.md` -- Project documentation and usage examples

### Key Functions in src/lib.rs
- `generate_codes()` -- Creates unique bit pattern codes
- `generate_arc_arguments()` -- Creates ImageMagick drawing arguments
- `generate_targets()` -- Main image generation function
- `calc_parity()`, `count_bit_transitions()` -- Mathematical utility functions

### Output
- Generates PNG files named with their code values (e.g., `65.png`, `71.png`)
- Files are 16-bit grayscale PNG images
- Default generates 147 unique codes for 12-bit encoding
- Each image contains circular targets with encoded bit patterns for photogrammetry

## Dependencies

### Rust Dependencies
- clap 4.0+ (command-line interface) - specified in Cargo.toml
- Rust 1.70+ (tested and working)

### System Dependencies
- ImageMagick 6.x or 7.x (for image generation)
- Cross-platform: Works on Linux, macOS, and Windows

### Binary Distribution
The compiled binary is self-contained except for ImageMagick:
- No runtime interpreter needed (unlike Python)
- Single executable file
- Cross-platform compatibility

## Common Issues

### ImageMagick Integration
- **Direct process invocation**: Uses `magick` command directly (no shell dependencies)
- **Cross-platform**: Handles Windows, macOS, and Linux differences automatically
- Application expects `magick` command to be available in PATH

### Build Issues
- Use `cargo clean` and rebuild if encountering cache issues
- Check Rust version with `rustc --version` (requires 1.70+)
- Use `cargo update` to update dependencies if needed

### Performance
- Image generation is CPU-intensive but faster than Python version
- Memory usage is minimal due to Rust's efficiency
- No parallel processing implemented (generates images sequentially)
- Binary startup is nearly instantaneous compared to Python