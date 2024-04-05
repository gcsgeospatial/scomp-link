import math
import click
import subprocess
import os
import shutil


def bitwise_rotate_left(val, bits, total_bits):
    """
    Perform a bitwise rotation to the left.

    Parameters:
    val (int): The value to rotate.
    bits (int): The number of bits to rotate.
    total_bits (int): The total number of bits in the value.

    Returns:
    int: The result of the bitwise rotation.
    """
    return (val << bits) & (2**total_bits - 1) | (
        (val & (2**total_bits - 1)) >> total_bits - bits
    )


def find_smallest_rotation(val, total_bits):
    """
    Find the smallest representation of a value through bitwise rotations.

    Parameters:
    val (int): The value to find the smallest rotation for.
    total_bits (int): The total number of bits in the value.

    Returns:
    int: The smallest rotated value.
    """
    smallest = val
    for i in range(1, total_bits):
        smallest = min(bitwise_rotate_left(val, i, total_bits), smallest)
    return smallest


def calc_parity(val):
    """
    Determine the parity of a given value.

    Parameters:
    val (int): The value to calculate parity for.

    Returns:
    bool: True if even parity, False otherwise.
    """
    parity = True
    while val:
        parity = not parity
        val = val & (val - 1)
    return parity


def count_bit_transitions(val):
    """
    Count the number of transitions between bits in a value.

    Parameters:
    val (int): The value to count bit transitions for.

    Returns:
    int: The number of bit transitions.
    """
    transitions = 0
    prev_bit = 0
    while val:
        new_bit = val & 1
        if new_bit > prev_bit:
            transitions += 1
        prev_bit = new_bit
        val >>= 1
    return transitions


def generate_codes(bits, transitions=None, max_codes=None):
    """
    Generate unique codes based on a given number of bits and optional constraints.

    Parameters:
    bits (int): The number of bits to encode.
    transitions (int, optional): The optional number of bit transitions. Default is None.
    max_codes (int, optional): The maximum number of codes to generate. Default is None.

    Returns:
    list: A list of unique generated codes.
    """
    codes = []
    # Codes all start with 0 and end with 1, allowing us to check fewer numbers
    for i in range(2 ** (bits - 2)):
        # Add 1 bit to end
        code = (i << 1) + 1

        # Perform cyclic shift to minimize value
        code = find_smallest_rotation(code, bits)

        # Check which pairs of opposite segments are both 1
        half_bits = bits >> 1
        diff = (code & (2**half_bits - 1)) & (
            (code & ((2**half_bits - 1) << half_bits)) >> half_bits
        )

        # Find parity
        parity = calc_parity(code)

        # Count number of transitions
        num_transitions = count_bit_transitions(code) if transitions else None

        # Find unique codes with even parity and at least one pair of opposite
        # segments that are both 1 (and correct number of transitions,
        # if applicable)
        if (
            parity
            and diff > 0
            and (transitions is None or num_transitions == transitions)
            and code not in codes
        ):
            codes.append(code)
            if max_codes is not None and len(codes) >= max_codes:
                break

    return codes


def angle_to_coordinates(angle, radius, center):
    """
    Convert polar coordinates to Cartesian coordinates.

    Parameters:
    angle (float): The angle in degrees.
    radius (float): The radius.
    center (tuple): The center point as a tuple (x, y).

    Returns:
    tuple: The Cartesian coordinates (x, y).
    """
    x = center[0] + radius * math.sin(math.radians(angle))
    y = center[1] - radius * math.cos(math.radians(angle))
    return (int(x), int(y))


def generate_arc_commands(code, bits, center, radius_outer):
    """
    Generate ImageMagick commands for drawing arcs based on the code.

    Parameters:
    code (int): The code to generate arcs for.
    bits (int): The number of bits used for encoding.
    center (tuple): The center point as a tuple (x, y).
    radius_outer (float): The radius of the outer circle.

    Returns:
    str: ImageMagick commands for drawing arcs.
    """
    commands = []
    angle_per_segment = 360 / bits
    for i in range(bits):
        if (1 << (bits - 1 - i)) & code:
            start_angle = i * angle_per_segment
            end_angle = (i + 1) * angle_per_segment

            # Calculate start and end points for the outer and inner arcs
            start_outer = angle_to_coordinates(start_angle, radius_outer, center)
            end_outer = angle_to_coordinates(end_angle, radius_outer, center)

            # Construct the arc command
            # Note: This assumes clockwise drawing; adjust as necessary
            command = f"-fill white -draw \"path 'M {center[0]},{center[1]} L {start_outer[0]},{start_outer[1]} A {radius_outer},{radius_outer} 0 0,1 {end_outer[0]},{end_outer[1]} Z'\""
            commands.append(command)

    return " ".join(commands)


@click.command()
@click.option("--bits", default=12, help="Number of bits to encode.")
@click.option(
    "--output-dir", default=".", help="Directory where PNG files will be written."
)
@click.option("--radius-inner-dot", default=24, help="Radius of the inner dot.")
@click.option(
    "--radius-inner-black", default=288, help="Radius of the inner black circle."
)
@click.option(
    "--radius-outer-white", default=660, help="Radius of the outer white circle."
)
@click.option(
    "--radius-outer-black", default=1032, help="Radius of the outer black circle."
)
@click.option("--width", default=3000, help="Width of the PNG.")
@click.option("--height", default=3000, help="Height of the PNG.")
@click.option(
    "--transitions", default=None, type=int, help="Optional number of bit transitions."
)
@click.option(
    "--max-codes", default=None, type=int, help="Maximum number of codes to generate."
)
def generate_targets(
    bits,
    output_dir,
    radius_inner_dot,
    radius_inner_black,
    radius_outer_white,
    radius_outer_black,
    width,
    height,
    transitions,
    max_codes,
):
    """
    Generate target images with specified parameters and save them as PNG files.

    Parameters:
    bits (int): Number of bits to encode.
    output_dir (str): Directory where PNG files will be saved.
    radius_inner_dot (int): Radius of the inner dot.
    radius_inner_black (int): Radius of the inner black circle.
    radius_outer_white (int): Radius of the outer white circle.
    radius_outer_black (int): Radius of the outer black circle.
    width (int): Width of the PNG.
    height (int): Height of the PNG.
    transitions (int, optional): Number of bit transitions. Default is None.
    max_codes (int, optional): Maximum number of codes to generate. Default is None.
    """
    try:
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
    except OSError as e:
        click.echo(f"Error creating output directory: {e}")
        return

    if not shutil.which("magick"):
        click.echo("Error: ImageMagick is not installed or not in the system's PATH.")
        return

    if bits <= 0 or bits % 2 != 0:
        click.echo("Error: Number of bits must be positive and even.")
        return

    codes = generate_codes(bits, transitions, max_codes)
    click.echo(codes)

    for code in codes:
        # Example usage within your existing function, assuming center is the middle of your canvas
        # Example center and radii, adjust as necessary
        center = (width / 2, height / 2)
        radius_outer = (
            radius_outer_black + 2
        )  # Example outer radius, adjust based on your needs

        bits = 12
        arc_commands = generate_arc_commands(
            code, bits, center, radius_outer
        )

        filename = f"{output_dir}/{code}.png"
        # Construct the ImageMagick command here using the provided radii and other parameters
        # This is a placeholder for how you might start constructing the command
        outer_black_circle = f'-fill black -draw "circle {center[0]},{center[1]} {center[0]},{center[1]+radius_outer_black}"'
        outer_white_circle = f'-fill white -draw "circle {center[0]},{center[1]} {center[0]},{center[1]+radius_outer_white}"'
        inner_black_circle = f'-fill black -draw "circle {center[0]},{center[1]} {center[0]},{center[1]+radius_inner_black}"'
        inner_white_circle = f'-fill white -draw "circle {center[0]},{center[1]} {center[0]},{center[1]+radius_inner_dot}"'
        command = f"magick -size {width}x{height} xc:white {outer_black_circle} {arc_commands} {outer_white_circle} {inner_black_circle} {inner_white_circle} {filename}"
        # Add more parameters to the command based on the radii and the code
        # click.echo(command)

        # Execute the command
        subprocess.run(command, shell=True)
        click.echo(f"Generated {filename}")


if __name__ == "__main__":
    generate_targets()
