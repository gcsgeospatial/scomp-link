# Examples

This document provides basic examples for using scomp-link to generate photogrammetry targets.

## Basic Examples

### Quick Start: Generate Test Targets

For your first time using scomp-link, start with a small test:

```bash
# Generate 3 small targets for testing
python main.py --bits 12 --output-dir ./test --width 500 --height 500 --max-codes 3

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
python main.py --bits 12 --output-dir ./production --width 3000 --height 3000

# Each file will be approximately 2-4 MB in size
```

### Custom Target Dimensions

Adjust target geometry for specific camera systems or measurement requirements:

```bash
# Larger targets for long-distance photography
python main.py --bits 12 \
  --radius-inner-dot 50 \
  --radius-inner-black 400 \
  --radius-outer-white 900 \
  --radius-outer-black 1400 \
  --width 4000 --height 4000 \
  --output-dir ./large-targets

# Smaller, high-density targets for close-range work
python main.py --bits 12 \
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
python main.py --bits 12 --max-codes 10 --output-dir ./test

# Generate small images for prototyping
python main.py --bits 12 --width 500 --height 500 --max-codes 5
```

### Advanced Filtering

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

## Troubleshooting

### Common Issues

**ImageMagick not found:**
```bash
Error: ImageMagick is not installed or not in the system's PATH.
```
*Solution:* Install ImageMagick or create symlink for `magick` command (see installation instructions in README).

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