"""
Tests for main.py functions.

This module contains comprehensive tests for all functions in main.py,
including bitwise operations, code generation, and geometric calculations.
"""

import math
import tempfile
from unittest.mock import MagicMock, patch

import pytest
from click.testing import CliRunner

import main


class TestBitwiseOperations:
    """Test bitwise operations functions."""

    def test_bitwise_rotate_left_basic(self):
        """Test basic bitwise rotation to the left."""
        # Test 8-bit rotation: 10110000 (176) rotated left by 2 bits
        # Should become: 11000010 (194)
        result = main.bitwise_rotate_left(176, 2, 8)
        assert result == 194

    def test_bitwise_rotate_left_full_rotation(self):
        """Test full rotation returns original value."""
        val = 170  # 10101010
        bits = 8
        result = main.bitwise_rotate_left(val, bits, bits)
        assert result == val

    def test_bitwise_rotate_left_zero_rotation(self):
        """Test zero rotation returns original value."""
        val = 170
        result = main.bitwise_rotate_left(val, 0, 8)
        assert result == val

    def test_bitwise_rotate_left_single_bit(self):
        """Test rotation with single bit."""
        # 1 rotated left by 1 in 2 bits becomes 2
        result = main.bitwise_rotate_left(1, 1, 2)
        assert result == 2

    def test_find_smallest_rotation_basic(self):
        """Test finding smallest rotation of a value."""
        # For 12-bit value 2816 (101100000000), smallest rotation should be smaller
        result = main.find_smallest_rotation(2816, 12)
        assert result <= 2816

    def test_find_smallest_rotation_already_smallest(self):
        """Test when value is already smallest."""
        # 1 should be the smallest rotation
        result = main.find_smallest_rotation(1, 8)
        assert result == 1

    def test_find_smallest_rotation_symmetric(self):
        """Test with symmetric pattern."""
        # 10101010 in 8 bits should return same or smaller
        val = 170  # 10101010
        result = main.find_smallest_rotation(val, 8)
        assert result <= val


class TestParityAndTransitions:
    """Test parity and bit transition functions."""

    def test_calc_parity_even(self):
        """Test even parity calculation."""
        # 0 has even parity (0 set bits)
        assert main.calc_parity(0) is True

        # 3 (011) has even parity (2 set bits)
        assert main.calc_parity(3) is True

        # 15 (1111) has even parity (4 set bits)
        assert main.calc_parity(15) is True

    def test_calc_parity_odd(self):
        """Test odd parity calculation."""
        # 1 has odd parity (1 set bit)
        assert main.calc_parity(1) is False

        # 7 (111) has odd parity (3 set bits)
        assert main.calc_parity(7) is False

    def test_count_bit_transitions_basic(self):
        """Test basic bit transition counting."""
        # 1010 has 2 transitions (0->1, 0->1) - only counts 0->1 transitions
        result = main.count_bit_transitions(10)  # 1010 in binary
        assert result == 2

    def test_count_bit_transitions_no_transitions(self):
        """Test no bit transitions."""
        # All zeros has 0 transitions
        assert main.count_bit_transitions(0) == 0

        # Single bit has 1 transition (0->1)
        assert main.count_bit_transitions(1) == 1

    def test_count_bit_transitions_alternating(self):
        """Test alternating bit pattern."""
        # 0101 has 2 transitions (0->1, 0->1) - only counts 0->1 transitions
        result = main.count_bit_transitions(5)  # 101 in binary
        assert result == 2


class TestCodeGeneration:
    """Test code generation functions."""

    def test_generate_codes_basic(self):
        """Test basic code generation."""
        codes = main.generate_codes(4, max_codes=5)

        # Should return a list
        assert isinstance(codes, list)

        # Should not exceed max_codes
        assert len(codes) <= 5

        # All codes should be unique
        assert len(codes) == len(set(codes))

    def test_generate_codes_with_transitions(self):
        """Test code generation with transition constraint."""
        codes = main.generate_codes(6, transitions=2, max_codes=3)

        # Verify each code has correct number of transitions
        for code in codes:
            assert main.count_bit_transitions(code) == 2

    def test_generate_codes_parity_constraint(self):
        """Test that all generated codes have even parity."""
        codes = main.generate_codes(8, max_codes=10)

        for code in codes:
            assert main.calc_parity(code) is True

    def test_generate_codes_invalid_bits_odd(self):
        """Test with odd number of bits."""
        # Should handle odd bits gracefully
        codes = main.generate_codes(5, max_codes=3)
        assert isinstance(codes, list)

    def test_generate_codes_invalid_bits_zero(self):
        """Test with zero bits."""
        # Zero bits should cause an error or return empty list
        # The current implementation will fail with 2**(0-2) = 2**(-2) = 0.25
        with pytest.raises((TypeError, ValueError)):
            main.generate_codes(0, max_codes=3)


class TestGeometry:
    """Test geometric calculation functions."""

    def test_angle_to_coordinates_basic(self):
        """Test basic angle to coordinates conversion."""
        center = (100, 100)
        radius = 50

        # 0 degrees should point straight up
        x, y = main.angle_to_coordinates(0, radius, center)
        assert x == 100
        assert y == 50  # y decreases going up

        # 90 degrees should point right
        x, y = main.angle_to_coordinates(90, radius, center)
        assert abs(x - 150) < 1  # Allow small floating point error
        assert abs(y - 100) < 1

    def test_angle_to_coordinates_full_circle(self):
        """Test coordinates for full circle."""
        center = (0, 0)
        radius = 10

        # Test various angles
        angles = [0, 90, 180, 270]
        for angle in angles:
            x, y = main.angle_to_coordinates(angle, radius, center)
            # Distance from center should equal radius (within rounding error)
            distance = math.sqrt(x * x + y * y)
            assert abs(distance - radius) < 1


class TestArcCommands:
    """Test arc command generation functions."""

    def test_generate_arc_commands_basic(self):
        """Test basic arc command generation."""
        code = 7  # 111 in binary
        bits = 3
        center = (100, 100)
        radius = 50

        commands = main.generate_arc_commands(code, bits, center, radius)

        # Should return a string
        assert isinstance(commands, str)

        # Should contain ImageMagick draw commands
        assert "fill white" in commands
        assert "draw" in commands
        assert "path" in commands

    def test_generate_arc_commands_empty_code(self):
        """Test arc commands with zero code."""
        commands = main.generate_arc_commands(0, 8, (100, 100), 50)

        # Should return empty string for zero code
        assert commands == ""

    def test_generate_arc_commands_full_code(self):
        """Test arc commands with all bits set."""
        bits = 4
        code = (1 << bits) - 1  # All bits set
        commands = main.generate_arc_commands(code, bits, (100, 100), 50)

        # Should contain multiple arc commands
        assert commands.count("fill white") == bits


@patch("main.shutil.which")
@patch("main.subprocess.run")
@patch("main.os.makedirs")
@patch("main.os.path.exists")
class TestGenerateTargetsCommand:
    """Test the main CLI command function."""

    def test_generate_targets_basic(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test basic target generation."""
        # Mock dependencies
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = True
        mock_subprocess.return_value = MagicMock()

        runner = CliRunner()

        with tempfile.TemporaryDirectory() as temp_dir:
            result = runner.invoke(
                main.generate_targets,
                ["--bits", "4", "--output-dir", temp_dir, "--max-codes", "1"],
            )

            # Should execute successfully
            assert result.exit_code == 0

    def test_generate_targets_missing_imagemagick(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test when ImageMagick is not available."""
        mock_which.return_value = None  # ImageMagick not found
        mock_exists.return_value = True

        runner = CliRunner()
        result = runner.invoke(main.generate_targets, ["--bits", "4"])

        # Should show error about missing ImageMagick
        assert "ImageMagick is not installed" in result.output

    def test_generate_targets_invalid_bits_odd(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test with odd number of bits."""
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = True

        runner = CliRunner()
        result = runner.invoke(main.generate_targets, ["--bits", "3"])

        # Should show error about odd bits
        assert "must be positive and even" in result.output

    def test_generate_targets_invalid_bits_zero(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test with zero bits."""
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = True

        runner = CliRunner()
        result = runner.invoke(main.generate_targets, ["--bits", "0"])

        # Should show error about invalid bits
        assert "must be positive and even" in result.output

    def test_generate_targets_directory_creation(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test directory creation when output directory doesn't exist."""
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = False  # Directory doesn't exist
        mock_makedirs.return_value = None
        mock_subprocess.return_value = MagicMock()

        runner = CliRunner()
        runner.invoke(
            main.generate_targets,
            ["--bits", "4", "--output-dir", "/tmp/test_output", "--max-codes", "1"],
        )

        # Should call makedirs
        mock_makedirs.assert_called_once_with("/tmp/test_output")

    def test_generate_targets_directory_creation_error(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test error handling when directory creation fails."""
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = False
        mock_makedirs.side_effect = OSError("Permission denied")

        runner = CliRunner()
        result = runner.invoke(
            main.generate_targets, ["--bits", "4", "--output-dir", "/tmp/test_output"]
        )

        # Should show error about directory creation
        assert "Error creating output directory" in result.output

    def test_generate_targets_imagemagick_command_format(
        self, mock_exists, mock_makedirs, mock_subprocess, mock_which
    ):
        """Test that ImageMagick command has correct xc:white syntax."""
        mock_which.return_value = "/usr/bin/magick"
        mock_exists.return_value = True
        mock_subprocess.return_value = MagicMock()

        runner = CliRunner()
        result = runner.invoke(
            main.generate_targets,
            ["--bits", "4", "--output-dir", "/tmp/test", "--max-codes", "1"],
        )

        # Should execute successfully
        assert result.exit_code == 0

        # Verify subprocess.run was called
        assert mock_subprocess.called

        # Get the command that was executed
        call_args = mock_subprocess.call_args
        command = call_args[0][0]  # First positional argument

        # Verify the command contains correct xc:white syntax (no space)
        assert "xc:white" in command
        # Verify it doesn't contain the incorrect syntax with space
        assert "xc: white" not in command


class TestIntegration:
    """Integration tests that test multiple functions together."""

    def test_full_code_generation_pipeline(self):
        """Test the complete code generation pipeline."""
        # Generate codes
        codes = main.generate_codes(6, max_codes=3)

        # Verify all codes meet requirements
        for code in codes:
            # Even parity
            assert main.calc_parity(code) is True

            # Check opposite segments requirement
            half_bits = 6 >> 1
            diff = (code & (2**half_bits - 1)) & (
                (code & ((2**half_bits - 1) << half_bits)) >> half_bits
            )
            assert diff > 0

            # Should be minimal rotation
            minimal = main.find_smallest_rotation(code, 6)
            assert code == minimal

    def test_geometric_calculations_consistency(self):
        """Test that geometric calculations are consistent."""
        center = (200, 200)
        radius = 100

        # Test that angles 180 degrees apart have coordinates that are center +/- radius
        x1, y1 = main.angle_to_coordinates(0, radius, center)
        x2, y2 = main.angle_to_coordinates(180, radius, center)

        # Y coordinates should be symmetric around center
        assert abs((y1 - center[1]) + (y2 - center[1])) < 2  # Allow rounding error

        # X coordinates should be the same (pointing straight up/down)
        assert abs(x1 - x2) < 2


if __name__ == "__main__":
    pytest.main([__file__])
