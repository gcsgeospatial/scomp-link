# Examples and Workflows

This document provides comprehensive examples and common workflows for using scomp-link to generate photogrammetry targets.

## Table of Contents

- [Basic Examples](#basic-examples)
- [Common Workflows](#common-workflows)
- [Advanced Use Cases](#advanced-use-cases)
- [Integration Examples](#integration-examples)
- [Troubleshooting Scenarios](#troubleshooting-scenarios)

## Basic Examples

### Quick Start: Generate Test Targets

For your first time using scomp-link, start with a small test:

```bash
# Generate 3 small targets for testing (completes in ~2 seconds)
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

# This will take approximately 107 seconds and create 147 PNG files
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

## Common Workflows

### Workflow 1: Camera Calibration Setup

Generate a set of targets for camera calibration:

```bash
# Step 1: Create calibration targets (moderate count for manageable printing)
python main.py --bits 12 --max-codes 25 --output-dir ./calibration \
  --width 2000 --height 2000

# Step 2: Verify target quality
ls -la calibration/ | wc -l  # Should show 25 files
file calibration/*.png       # Verify PNG format

# Step 3: Print targets at known scale and arrange on calibration board
# Recommended: Print at 300 DPI for precise measurements
```

### Workflow 2: Batch Processing for Different Scales

Generate targets at multiple scales for multi-resolution photogrammetry:

```bash
# Small scale (close-range, high precision)
python main.py --bits 12 --max-codes 20 \
  --width 1500 --height 1500 \
  --radius-outer-black 600 \
  --output-dir ./scale-small

# Medium scale (standard applications)  
python main.py --bits 12 --max-codes 20 \
  --width 3000 --height 3000 \
  --radius-outer-black 1032 \
  --output-dir ./scale-medium

# Large scale (long-distance photography)
python main.py --bits 12 --max-codes 20 \
  --width 5000 --height 5000 \
  --radius-outer-black 1800 \
  --output-dir ./scale-large
```

### Workflow 3: Quality-Controlled Target Generation

Generate targets with specific bit transition patterns for improved detection:

```bash
# Generate targets with exactly 3 bit transitions (good for edge detection)
python main.py --bits 12 --transitions 3 --output-dir ./transitions-3

# Generate targets with 4 bit transitions (balanced pattern complexity)
python main.py --bits 12 --transitions 4 --output-dir ./transitions-4

# Generate targets with 2 bit transitions (simple patterns, high contrast)
python main.py --bits 12 --transitions 2 --output-dir ./transitions-2

# Compare the number of targets generated
echo "3 transitions: $(ls transitions-3/ | wc -l) targets"
echo "4 transitions: $(ls transitions-4/ | wc -l) targets"  
echo "2 transitions: $(ls transitions-2/ | wc -l) targets"
```

## Advanced Use Cases

### Use Case 1: High-Precision Measurement Setup

For precision measurement applications requiring sub-pixel accuracy:

```bash
# Generate high-resolution targets with precise geometry
python main.py --bits 16 \
  --width 6000 --height 6000 \
  --radius-inner-dot 40 \
  --radius-inner-black 480 \
  --radius-outer-white 1100 \
  --radius-outer-black 1720 \
  --max-codes 50 \
  --output-dir ./high-precision

# Recommended printing: 600 DPI on high-quality matte paper
# Recommended placement: Rigid mounting with precise positioning
```

### Use Case 2: Multi-Camera System Calibration

Generate targets optimized for multi-camera stereo systems:

```bash
# Set 1: Primary targets (high contrast, easy detection)
python main.py --bits 12 --transitions 2 \
  --max-codes 15 \
  --output-dir ./stereo-primary

# Set 2: Secondary targets (different patterns, validation)
python main.py --bits 12 --transitions 4 \
  --max-codes 15 \
  --output-dir ./stereo-secondary

# Recommended usage: Alternate primary/secondary targets in checkerboard pattern
```

### Use Case 3: Automated Quality Control

Generate targets with validation for automated systems:

```bash
# Create test batch
python main.py --bits 12 --max-codes 5 --output-dir ./validation \
  --width 1000 --height 1000

# Validate target properties
for file in validation/*.png; do
    echo "Checking $file..."
    # Verify file format
    file "$file" | grep -q "PNG" || echo "ERROR: Not PNG format"
    
    # Verify file size (should be reasonable)
    size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null)
    if [ "$size" -lt 100000 ]; then
        echo "WARNING: File $file unusually small ($size bytes)"
    fi
done
```

## Integration Examples

### Integration with Python Scripts

```python
#!/usr/bin/env python3
"""
Example: Generate targets programmatically
"""
import subprocess
import os

def generate_target_set(name, bits=12, count=10, size=2000):
    """Generate a set of targets with specified parameters."""
    output_dir = f"./targets-{name}"
    
    cmd = [
        "python", "main.py",
        "--bits", str(bits),
        "--max-codes", str(count), 
        "--width", str(size),
        "--height", str(size),
        "--output-dir", output_dir
    ]
    
    print(f"Generating {count} targets in {output_dir}...")
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode == 0:
        files = os.listdir(output_dir)
        print(f"Successfully generated {len(files)} targets")
        return True
    else:
        print(f"Error: {result.stderr}")
        return False

# Generate different target sets
generate_target_set("calibration", bits=12, count=20, size=3000)
generate_target_set("validation", bits=12, count=5, size=1500)
generate_target_set("production", bits=16, count=50, size=4000)
```

### Integration with Makefile

```makefile
# Makefile for automated target generation

.PHONY: all clean test calibration production

# Default target
all: test calibration production

# Quick test generation
test:
	python main.py --bits 12 --max-codes 3 --width 500 --height 500 \
		--output-dir ./test-targets
	@echo "Test targets generated in ./test-targets/"

# Calibration targets
calibration:
	python main.py --bits 12 --max-codes 25 --width 2000 --height 2000 \
		--output-dir ./calibration-targets
	@echo "Calibration targets generated in ./calibration-targets/"

# Production targets  
production:
	python main.py --bits 12 --width 3000 --height 3000 \
		--output-dir ./production-targets
	@echo "Production targets generated in ./production-targets/"

# Specialized targets with transitions
filtered:
	python main.py --bits 12 --transitions 3 --max-codes 20 \
		--width 2500 --height 2500 --output-dir ./filtered-targets
	@echo "Filtered targets generated in ./filtered-targets/"

# Clean generated files
clean:
	rm -rf ./test-targets ./calibration-targets ./production-targets ./filtered-targets
	@echo "Cleaned all generated target directories"

# Validate setup
validate:
	python -c "import main; print('Import successful')"
	python main.py --help > /dev/null
	@echo "Setup validation passed"
```

### Integration with Shell Scripts

```bash
#!/bin/bash
# generate_target_suite.sh - Generate complete target suite

set -e  # Exit on any error

echo "=== scomp-link Target Generation Suite ==="

# Configuration
BASE_DIR="./photogrammetry-targets"
DATE=$(date +%Y%m%d)
SUITE_DIR="${BASE_DIR}/${DATE}"

# Create directory structure
mkdir -p "${SUITE_DIR}"/{test,calibration,production,filtered}

echo "Generating target suite in: ${SUITE_DIR}"

# Generate test targets (fast)
echo "1/4 Generating test targets..."
python main.py --bits 12 --max-codes 5 --width 500 --height 500 \
    --output-dir "${SUITE_DIR}/test"

# Generate calibration targets
echo "2/4 Generating calibration targets..."
python main.py --bits 12 --max-codes 25 --width 2000 --height 2000 \
    --output-dir "${SUITE_DIR}/calibration"

# Generate filtered targets
echo "3/4 Generating filtered targets..."
python main.py --bits 12 --transitions 3 --max-codes 15 \
    --width 2500 --height 2500 \
    --output-dir "${SUITE_DIR}/filtered"

# Generate production targets (slow)
echo "4/4 Generating production targets (this may take several minutes)..."
python main.py --bits 12 --width 3000 --height 3000 \
    --output-dir "${SUITE_DIR}/production"

# Generate summary
echo ""
echo "=== Generation Complete ==="
echo "Target suite generated in: ${SUITE_DIR}"
echo "Summary:"
echo "  Test targets:       $(ls "${SUITE_DIR}/test" | wc -l)"
echo "  Calibration targets: $(ls "${SUITE_DIR}/calibration" | wc -l)"
echo "  Filtered targets:    $(ls "${SUITE_DIR}/filtered" | wc -l)"
echo "  Production targets:  $(ls "${SUITE_DIR}/production" | wc -l)"

# Calculate total size
total_size=$(du -sh "${SUITE_DIR}" | cut -f1)
echo "  Total size:         ${total_size}"

echo ""
echo "Ready for photogrammetry applications!"
```

## Troubleshooting Scenarios

### Scenario 1: ImageMagick Installation Issues

**Problem:** `Error: ImageMagick is not installed or not in the system's PATH.`

**Solutions by platform:**

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install imagemagick
sudo ln -sf /usr/bin/convert-im6.q16 /usr/local/bin/magick

# macOS with Homebrew
brew install imagemagick

# Conda (any platform) - recommended
conda install -c conda-forge imagemagick

# Verify installation
magick --version
```

### Scenario 2: Performance Optimization

**Problem:** Target generation is too slow for your workflow.

**Solutions:**

```bash
# Use smaller image dimensions for testing
python main.py --bits 12 --width 1000 --height 1000 --max-codes 10

# Generate only needed targets
python main.py --bits 12 --max-codes 20  # Instead of all 147

# Use specific transition filters to reduce count
python main.py --bits 12 --transitions 3  # Generates fewer targets

# Consider parallel generation (advanced)
# Split generation across multiple processes
python main.py --bits 12 --max-codes 25 --output-dir ./batch1 &
python main.py --bits 12 --max-codes 25 --transitions 3 --output-dir ./batch2 &
wait
```

### Scenario 3: Quality Validation

**Problem:** Need to verify target quality before use.

**Validation script:**

```bash
#!/bin/bash
# validate_targets.sh

TARGET_DIR="$1"
if [ -z "$TARGET_DIR" ]; then
    echo "Usage: $0 <target_directory>"
    exit 1
fi

echo "Validating targets in: $TARGET_DIR"

error_count=0
for file in "$TARGET_DIR"/*.png; do
    if [ ! -f "$file" ]; then
        echo "ERROR: No PNG files found in $TARGET_DIR"
        exit 1
    fi
    
    # Check file format
    if ! file "$file" | grep -q "PNG image data"; then
        echo "ERROR: $file is not a valid PNG image"
        ((error_count++))
    fi
    
    # Check file size (should be reasonable)
    size=$(stat -c%s "$file" 2>/dev/null)
    if [ "$size" -lt 50000 ]; then
        echo "WARNING: $file is unusually small ($size bytes)"
    fi
    
    # Check dimensions using ImageMagick
    if command -v identify >/dev/null; then
        dims=$(identify -format "%wx%h" "$file")
        echo "  $file: $dims"
    fi
done

if [ $error_count -eq 0 ]; then
    echo "✓ All targets passed validation"
else
    echo "✗ Found $error_count errors"
    exit 1
fi
```

### Scenario 4: Custom Bit Patterns

**Problem:** Need specific bit patterns or codes.

**Advanced filtering:**

```python
#!/usr/bin/env python3
"""
Generate targets with custom bit pattern requirements
"""
import main

def custom_code_filter(codes, min_ones=4, max_ones=8):
    """Filter codes by number of '1' bits."""
    filtered = []
    for code in codes:
        ones_count = bin(code).count('1')
        if min_ones <= ones_count <= max_ones:
            filtered.append(code)
    return filtered

# Generate base codes
codes = main.generate_codes(12)
print(f"Generated {len(codes)} base codes")

# Apply custom filter
filtered_codes = custom_code_filter(codes, min_ones=5, max_ones=7)
print(f"Filtered to {len(filtered_codes)} codes with 5-7 ones")

# You could then generate images for specific codes
# by modifying the main script or calling ImageMagick directly
```

This examples document provides comprehensive guidance for users at all levels, from basic usage to advanced integration scenarios.