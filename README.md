# scomp-link

This code generates target images with specified parameters and saves them as PNG files. It utilizes the ImageMagick library for drawing arcs.

## Installation

### Prerequisites

This application requires ImageMagick to be installed on your system. Installation methods vary by platform:

### Linux/macOS

#### Method 1: Conda Environment (Recommended)
```bash
# Create conda environment with all dependencies including ImageMagick
conda env create -f environment.yaml
conda activate generate-targets
```

#### Method 2: System Package Manager + pip
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install imagemagick
pip install -r requirements.txt

# macOS (with Homebrew)
brew install imagemagick
pip install -r requirements.txt
```

### Windows

ImageMagick is not available via conda on Windows, so you need to install it separately:

#### Method 1: Direct Download (Recommended)
1. Download ImageMagick from the official website: https://imagemagick.org/script/download.php#windows
2. Choose the appropriate version for your system (32-bit or 64-bit)
3. Run the installer and ensure "Install development headers and libraries for C and C++" is checked
4. Add ImageMagick to your system PATH during installation
5. Install Python dependencies:
   ```cmd
   # Use the Windows-specific environment file
   conda env create -f environment-windows.yaml
   conda activate generate-targets
   
   # Or use pip
   pip install -r requirements.txt
   ```

#### Method 2: Chocolatey Package Manager
```cmd
# Install Chocolatey if not already installed (see https://chocolatey.org/install)
choco install imagemagick

# Install Python dependencies
pip install -r requirements.txt
```

#### Method 3: Scoop Package Manager
```cmd
# Install Scoop if not already installed (see https://scoop.sh/)
scoop install imagemagick

# Install Python dependencies
pip install -r requirements.txt
```

### Verifying Installation

After installation, verify that ImageMagick is properly installed and accessible:

```bash
# This should display ImageMagick version information
magick -version
```

If the command is not found, ensure that ImageMagick is properly added to your system's PATH environment variable.

## Usage

To use the code, follow these steps:

1. Install the required dependencies following the platform-specific instructions in the [Installation](#installation) section above.
2. Run the script `main.py` with appropriate command-line options to specify the parameters for generating the target images.
3. Generated PNG files will be saved in the specified output directory.

## Command-line Options

- `bits`: Number of bits to encode.
- `output-dir`: Directory where PNG files will be saved.
- `radius-inner-dot`: Radius of the inner dot.
- `radius-inner-black`: Radius of the inner black circle.
- `radius-outer-white`: Radius of the outer white circle.
- `radius-outer-black`: Radius of the outer black circle.
- `width`: Width of the PNG.
- `height`: Height of the PNG.
- `transitions`: Optional number of bit transitions.
- `max-codes`: Maximum number of codes to generate.

## Example Command

```bash
python main.py --bits 12 --output-dir ./output --width 3000 --height 3000
```

## Development

### Testing

This project includes a comprehensive test suite using pytest. To run the tests:

```bash
# Install development dependencies
pip install -r requirements.txt

# Run tests
python -m pytest test_main.py

# Run tests with coverage
python -m pytest test_main.py --cov=main --cov-report=term-missing
```

### Linting and Code Quality

The project uses several tools to maintain code quality:

```bash
# Check code style with flake8
python -m flake8 .

# Format code with black
python -m black .

# Sort imports with isort
python -m isort .
```

### Continuous Integration

GitHub Actions automatically runs tests and linting on all pull requests and pushes to main/develop branches. The workflow tests against multiple Python versions (3.8-3.12) to ensure compatibility.

## Acknowledgments

This code is based on the work of Christoph T. Schneider, as described in:

Schneider, C. T. “3-D Vermessung von Oberflächen und Bauteilen durch Photogrammetrie und Bildverarbeitung.” Proc. IDENT/VISION 91 (1991): 14-17.

An alternate implementation of similar functionality was provided by Matthew Petroff in his repository:

- [Photogrammetry Targets](https://mpetroff.net/2018/05/photogrammetry-targets/)
- [GitHub Repository](https://github.com/mpetroff)

Petroff's implementation is released into the public domain using the CC0 1.0 Public Domain Dedication.

## Notes

Petroff's script constructs SVG files for sheets of targets and then uses Inkscape to combine adjacent target segments into a single path and to export a PDF.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
