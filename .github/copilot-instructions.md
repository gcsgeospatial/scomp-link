# scomp-link - Photogrammetry Target Generator

scomp-link is a Python CLI application that generates target images with specified parameters for photogrammetry applications. It uses ImageMagick for drawing arcs and creating PNG files with circular targets encoded with specific bit patterns.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Method 1: Conda Environment (Recommended)
Bootstrap and run the application using conda (includes ImageMagick 7):
- `conda env create -f environment.yaml` -- creates conda environment. Takes 47 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `conda init bash` -- initialize conda for bash shell
- `source ~/.bashrc` -- reload bash configuration  
- `conda activate generate-targets` -- activate the environment
- `python main.py --help` -- verify installation and see available options

### Method 2: System Dependencies + pip
Alternative setup using system packages (requires ImageMagick 6 workaround):
- `sudo apt-get update` -- update package lists. Takes 30 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `sudo apt-get install -y imagemagick` -- install ImageMagick 6. Takes 60 seconds. NEVER CANCEL. Set timeout to 300+ seconds.
- `sudo ln -sf /usr/bin/convert-im6.q16 /usr/local/bin/magick` -- create symlink for magick command
- `export PATH="/usr/local/bin:$PATH"` -- add symlink to PATH
- `pip3 install -r requirements.txt` -- install Python dependencies (click). Takes 10 seconds.
- `python3 main.py --help` -- verify installation

## Running the Application

### Basic Usage
Run the application with default parameters:
- `python main.py --bits 12 --output-dir ./output --width 3000 --height 3000`

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
  - 10 codes at 3000x3000: ~7 seconds  
  - 50 codes at 3000x3000: ~36 seconds
  - 100 codes at 3000x3000: ~73 seconds  
  - 147 codes at 3000x3000: ~107 seconds (full generation)
- Set timeout to 300+ seconds for full generation runs
- Use `--max-codes` parameter to limit generation for testing

### Example Commands
Test with small parameters:
```bash
python main.py --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 5
```

Generate production targets:
```bash
python main.py --bits 12 --output-dir ./targets --width 3000 --height 3000
```

Generate with specific transitions:
```bash
python main.py --bits 12 --transitions 3 --output-dir ./filtered --width 3000 --height 3000 --max-codes 10
```

## Validation

### Always Test After Code Changes
Run these commands to verify functionality:
- `python -m py_compile main.py` -- check Python syntax
- `python -c "import main; print('Import successful')"` -- test import
- `python main.py --bits 12 --output-dir ./test --width 300 --height 300 --max-codes 2` -- basic functionality test
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

### Linting (Optional)
The code has style issues but is functionally correct:
- `pip3 install --user flake8 black` -- install linting tools
- `~/.local/bin/flake8 main.py` -- shows line length warnings (not critical)
- `~/.local/bin/black --check main.py` -- shows formatting issues (not critical)
- **DO NOT** run linting fixes unless specifically requested, as they may change functional code

## Project Structure

### Core Files
- `main.py` -- Main application with CLI interface and image generation logic
- `requirements.txt` -- Python dependencies (click==8.1.8)
- `environment.yaml` -- Conda environment specification with ImageMagick
- `README.md` -- Project documentation and usage examples

### Key Functions in main.py
- `generate_targets()` -- Main CLI entry point
- `generate_codes()` -- Creates unique bit pattern codes
- `generate_arc_commands()` -- Creates ImageMagick drawing commands
- Error handling for missing ImageMagick and invalid parameters

### Output
- Generates PNG files named with their code values (e.g., `65.png`, `71.png`)
- Files are 16-bit grayscale PNG images
- Default generates 147 unique codes for 12-bit encoding
- Each image contains circular targets with encoded bit patterns for photogrammetry

## Dependencies

### Python Dependencies
- click==8.1.8 (command-line interface)
- Python 3.12+ (tested and working)

### System Dependencies
- ImageMagick 6.x (system packages) OR ImageMagick 7.x (conda)
- Linux environment (tested on Ubuntu)

### Conda Environment
The conda environment automatically provides:
- Python with click
- ImageMagick 7.x with proper `magick` command
- All required system libraries

## Common Issues

### ImageMagick Version Differences
- **System ImageMagick 6**: Uses `convert` command, requires symlink to `magick`
- **Conda ImageMagick 7**: Uses `magick` command natively  
- Application expects `magick` command to be available

### Path Issues
- Conda activation may require `conda init bash` and `source ~/.bashrc`
- System symlink approach requires `export PATH="/usr/local/bin:$PATH"`

### Performance
- Image generation is CPU-intensive and scales linearly with number of codes
- Memory usage is minimal, disk space depends on image dimensions and count
- No parallel processing implemented (generates images sequentially)