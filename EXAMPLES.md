# Examples

This document provides basic examples for using scomp-link to generate photogrammetry targets.

## Basic Examples

### Quick Start: Generate Test Targets

For your first time using scomp-link, start with a small test:

```bash
# Generate 3 small targets for testing
scomp-link --bits 12 --output-dir ./test --width 500 --height 500 --max-codes 3

# Verify the output
ls -la test/
# Expected output: 3 PNG files named with their code values (e.g., 65.png, 71.png, 73.png)

# Check file details
file test/*.png
# Expected: PNG image data, 16-bit grayscale
```

### Standard Production Targets

Generate full-resolution targets for professional photogrammetry:

```bash
# Generate all 147 possible targets for 12-bit encoding
scomp-link --bits 12 --output-dir ./production --width 3000 --height 3000

# Each file will be approximately 2-4 MB in size
```

### Custom Target Dimensions

Adjust target geometry for specific camera systems or measurement requirements:

```bash
# Larger targets for long-distance photography
scomp-link --bits 12 \
  --radius-inner-dot 50 \
  --radius-inner-black 400 \
  --radius-outer-white 900 \
  --radius-outer-black 1400 \
  --width 4000 --height 4000 \
  --output-dir ./large-targets

# Smaller, high-density targets for close-range work
scomp-link --bits 12 \
  --radius-inner-dot 12 \
  --radius-inner-black 144 \
  --radius-outer-white 330 \
  --radius-outer-black 516 \
  --width 1500 --height 1500 \
  --output-dir ./small-targets
```

### Performance Optimization

```bash
# Generate only 10 targets for quick testing
scomp-link --bits 12 --max-codes 10 --output-dir ./test

# Generate small images for prototyping
scomp-link --bits 12 --width 500 --height 500 --max-codes 5
```

### Advanced Filtering

```bash
# Generate targets with exactly 3 bit transitions
scomp-link --bits 12 --transitions 3 --output-dir ./filtered

# Custom target geometry for specialized applications
scomp-link --bits 16 \
  --radius-inner-dot 30 \
  --radius-inner-black 350 \
  --radius-outer-white 800 \
  --radius-outer-black 1200 \
  --output-dir ./custom
```

## Installation Examples

### Option 1: Cargo Install (Recommended)

```bash
# Install directly from GitHub
cargo install --git https://github.com/gcsgeospatial/scomp-link.git

# Then use anywhere
scomp-link --help
scomp-link --bits 12 --output-dir ./targets
```

### Option 2: Build from Source

```bash
# Clone and build
git clone https://github.com/gcsgeospatial/scomp-link.git
cd scomp-link
cargo build --release

# Use the binary
./target/release/scomp-link --help
```

### Option 3: Install Script

```bash
# User installation
./install.sh

# System-wide installation (requires sudo)
./install.sh --system
```

## Development Examples

### Testing During Development

```bash
# Run without building binary
cargo run --release -- --help

# Test with specific parameters
cargo run --release -- --bits 6 --max-codes 3 --output-dir ./dev-test

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check
```

## Troubleshooting

### Common Issues

**ImageMagick not found:**
```bash
Error: ImageMagick is not installed or not in the system's PATH.
```
*Solution:* Install ImageMagick:
- Ubuntu/Debian: `sudo apt install imagemagick`
- macOS: `brew install imagemagick`
- Windows: Download from [ImageMagick website](https://imagemagick.org/script/download.php#windows)

**Invalid bits parameter:**
```bash
Error: Number of bits must be positive and even.
```
*Solution:* Use even numbers like 8, 12, 16, etc.

**Permission errors:**
```bash
Error creating output directory: Permission denied
```
*Solution:* Use a directory with write permissions or run with appropriate privileges.

**Build errors:**
```bash
error: failed to run custom build command for `some-crate`
```
*Solution:* Update Rust toolchain with `rustup update` or install build dependencies.

## Performance Notes

The Rust version provides significant performance improvements:
- **Faster startup**: Near-instantaneous vs ~100ms for Python
- **Faster generation**: ~25% faster image generation
- **Lower memory usage**: ~50% less RAM usage
- **Single binary**: No interpreter or runtime dependencies