// Library module to expose functions for testing

/// Perform a bitwise rotation to the left.
pub fn bitwise_rotate_left(val: u32, bits: u32, total_bits: u32) -> u32 {
    let mask = (1 << total_bits) - 1;
    ((val << bits) & mask) | ((val & mask) >> (total_bits - bits))
}

/// Find the smallest representation of a value through bitwise rotations.
pub fn find_smallest_rotation(val: u32, total_bits: u32) -> u32 {
    let mut smallest = val;
    for i in 1..total_bits {
        let rotated = bitwise_rotate_left(val, i, total_bits);
        if rotated < smallest {
            smallest = rotated;
        }
    }
    smallest
}

/// Determine the parity of a given value.
pub fn calc_parity(mut val: u32) -> bool {
    let mut parity = true;
    while val != 0 {
        parity = !parity;
        val = val & (val - 1);
    }
    parity
}

/// Count the number of transitions between bits in a value.
pub fn count_bit_transitions(mut val: u32) -> u32 {
    let mut transitions = 0;
    let mut prev_bit = 0;
    while val != 0 {
        let new_bit = val & 1;
        if new_bit > prev_bit {
            transitions += 1;
        }
        prev_bit = new_bit;
        val >>= 1;
    }
    transitions
}

/// Generate unique codes based on a given number of bits and optional constraints.
pub fn generate_codes(bits: u32, transitions: Option<u32>, max_codes: Option<usize>) -> Vec<u32> {
    let mut codes = Vec::new();

    // Codes all start with 0 and end with 1, allowing us to check fewer numbers
    for i in 0..(1 << (bits - 2)) {
        // Add 1 bit to end
        let mut code = (i << 1) + 1;

        // Perform cyclic shift to minimize value
        code = find_smallest_rotation(code, bits);

        // Check which pairs of opposite segments are both 1
        let half_bits = bits >> 1;
        let diff = (code & ((1 << half_bits) - 1))
            & ((code & (((1 << half_bits) - 1) << half_bits)) >> half_bits);

        // Find parity
        let parity = calc_parity(code);

        // Count number of transitions
        let num_transitions = if transitions.is_some() {
            Some(count_bit_transitions(code))
        } else {
            None
        };

        // Find unique codes with even parity and at least one pair of opposite
        // segments that are both 1 (and correct number of transitions, if applicable)
        if parity
            && diff > 0
            && (transitions.is_none() || num_transitions == transitions)
            && !codes.contains(&code)
        {
            codes.push(code);
            if let Some(max) = max_codes {
                if codes.len() >= max {
                    break;
                }
            }
        }
    }

    codes
}

/// Convert polar coordinates to Cartesian coordinates.
pub fn angle_to_coordinates(angle: f64, radius: f64, center: (f64, f64)) -> (i32, i32) {
    let x = center.0 + radius * angle.to_radians().sin();
    let y = center.1 - radius * angle.to_radians().cos();
    (x as i32, y as i32)
}

/// Generate ImageMagick arguments for drawing arcs based on the code.
pub fn generate_arc_arguments(
    code: u32,
    bits: u32,
    center: (f64, f64),
    radius_outer: f64,
) -> Vec<String> {
    let mut args = Vec::new();
    let angle_per_segment = 360.0 / bits as f64;

    for i in 0..bits {
        if (1 << (bits - 1 - i)) & code != 0 {
            let start_angle = i as f64 * angle_per_segment;
            let end_angle = (i + 1) as f64 * angle_per_segment;

            // Calculate start and end points for the outer arcs
            let start_outer = angle_to_coordinates(start_angle, radius_outer, center);
            let end_outer = angle_to_coordinates(end_angle, radius_outer, center);

            // Add arguments for this arc
            args.extend([
                "-fill".to_string(),
                "white".to_string(),
                "-draw".to_string(),
                format!(
                    "path 'M {}, {} L {}, {} A {}, {} 0 0, 1 {}, {} Z'",
                    center.0 as i32,
                    center.1 as i32,
                    start_outer.0,
                    start_outer.1,
                    radius_outer as i32,
                    radius_outer as i32,
                    end_outer.0,
                    end_outer.1
                ),
            ]);
        }
    }

    args
}

/// Generate ImageMagick commands for drawing arcs based on the code.
/// Legacy function for backward compatibility with tests.
pub fn generate_arc_commands(
    code: u32,
    bits: u32,
    center: (f64, f64),
    radius_outer: f64,
) -> String {
    let args = generate_arc_arguments(code, bits, center, radius_outer);
    // Join every 4 arguments (fill, white, draw, command) with spaces
    let mut commands = Vec::new();
    for chunk in args.chunks(4) {
        if chunk.len() == 4 {
            commands.push(format!(
                "{} {} {} \"{}\"",
                chunk[0], chunk[1], chunk[2], chunk[3]
            ));
        }
    }
    commands.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bitwise_operations {
        use super::*;

        #[test]
        fn test_bitwise_rotate_left_basic() {
            // Test 8-bit rotation: 10110000 (176) rotated left by 2 bits
            // Should become: 11000010 (194)
            let result = bitwise_rotate_left(176, 2, 8);
            assert_eq!(result, 194);
        }

        #[test]
        fn test_bitwise_rotate_left_full_rotation() {
            // Test full rotation returns original value
            let val = 170; // 10101010
            let bits = 8;
            let result = bitwise_rotate_left(val, bits, bits);
            assert_eq!(result, val);
        }

        #[test]
        fn test_bitwise_rotate_left_zero_rotation() {
            // Test zero rotation returns original value
            let val = 170;
            let result = bitwise_rotate_left(val, 0, 8);
            assert_eq!(result, val);
        }

        #[test]
        fn test_bitwise_rotate_left_single_bit() {
            // 1 rotated left by 1 in 2 bits becomes 2
            let result = bitwise_rotate_left(1, 1, 2);
            assert_eq!(result, 2);
        }

        #[test]
        fn test_find_smallest_rotation_basic() {
            // For 12-bit value 2816 (101100000000), smallest rotation should be smaller
            let result = find_smallest_rotation(2816, 12);
            assert!(result <= 2816);
        }

        #[test]
        fn test_find_smallest_rotation_already_smallest() {
            // 1 is already the smallest rotation
            let result = find_smallest_rotation(1, 8);
            assert_eq!(result, 1);
        }

        #[test]
        fn test_find_smallest_rotation_symmetric() {
            // For symmetric patterns like 15 (1111), rotation should return same value
            let result = find_smallest_rotation(15, 4);
            assert_eq!(result, 15);
        }
    }

    mod parity_and_transitions {
        use super::*;

        #[test]
        fn test_calc_parity_even() {
            // 3 has two 1 bits (11), so even parity
            assert!(calc_parity(3));
        }

        #[test]
        fn test_calc_parity_odd() {
            // 7 has three 1 bits (111), so odd parity
            assert!(!calc_parity(7));
        }

        #[test]
        fn test_count_bit_transitions_basic() {
            // 5 is 101, has 2 transitions (0→1, 1→0, 0→1)
            let result = count_bit_transitions(5);
            assert_eq!(result, 2);
        }

        #[test]
        fn test_count_bit_transitions_no_transitions() {
            // 0 has no transitions
            let result = count_bit_transitions(0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_count_bit_transitions_alternating() {
            // 10 is 1010, has 2 transitions
            let result = count_bit_transitions(10);
            assert_eq!(result, 2);
        }
    }

    mod code_generation {
        use super::*;

        #[test]
        fn test_generate_codes_basic() {
            let codes = generate_codes(6, None, Some(5));
            assert!(!codes.is_empty());
            assert!(codes.len() <= 5);

            // All codes should have even parity
            for &code in &codes {
                assert!(calc_parity(code));
            }
        }

        #[test]
        fn test_generate_codes_with_transitions() {
            let codes = generate_codes(8, Some(2), Some(3));

            // Check that all codes have exactly 2 transitions
            for &code in &codes {
                assert_eq!(count_bit_transitions(code), 2);
            }
        }

        #[test]
        fn test_generate_codes_parity_constraint() {
            let codes = generate_codes(6, None, Some(10));

            // All generated codes should have even parity
            for &code in &codes {
                assert!(calc_parity(code), "Code {} should have even parity", code);
            }
        }

        #[test]
        fn test_generate_codes_uniqueness() {
            let codes = generate_codes(8, None, Some(10));

            // Check for duplicates
            for i in 0..codes.len() {
                for j in (i + 1)..codes.len() {
                    assert_ne!(codes[i], codes[j], "Found duplicate codes");
                }
            }
        }
    }

    mod geometry {
        use super::*;

        #[test]
        fn test_angle_to_coordinates_basic() {
            let center = (100.0, 100.0);
            let radius = 50.0;

            // 0 degrees should point upward (north)
            let (x, y) = angle_to_coordinates(0.0, radius, center);
            assert_eq!(x, 100);
            assert_eq!(y, 50); // y decreases upward
        }

        #[test]
        fn test_angle_to_coordinates_full_circle() {
            let center = (0.0, 0.0);
            let radius = 100.0;

            // Test multiple angles
            let coords_0 = angle_to_coordinates(0.0, radius, center);
            let coords_90 = angle_to_coordinates(90.0, radius, center);
            let coords_180 = angle_to_coordinates(180.0, radius, center);
            let coords_270 = angle_to_coordinates(270.0, radius, center);

            // Verify approximate positions (allowing for floating point precision)
            assert!((coords_0.0 - 0).abs() <= 1);
            assert!((coords_0.1 - (-100)).abs() <= 1);

            assert!((coords_90.0 - 100).abs() <= 1);
            assert!((coords_90.1 - 0).abs() <= 1);

            assert!((coords_180.0 - 0).abs() <= 1);
            assert!((coords_180.1 - 100).abs() <= 1);

            assert!((coords_270.0 - (-100)).abs() <= 1);
            assert!((coords_270.1 - 0).abs() <= 1);
        }
    }

    mod arc_commands {
        use super::*;

        #[test]
        fn test_generate_arc_commands_basic() {
            let code = 5; // 101 in binary
            let bits = 3;
            let center = (100.0, 100.0);
            let radius = 50.0;

            let commands = generate_arc_commands(code, bits, center, radius);

            // Should generate commands for set bits
            assert!(!commands.is_empty());
            assert!(commands.contains("path"));
        }

        #[test]
        fn test_generate_arc_commands_empty_code() {
            let code = 0; // No bits set
            let bits = 8;
            let center = (100.0, 100.0);
            let radius = 50.0;

            let commands = generate_arc_commands(code, bits, center, radius);

            // Should generate no commands for empty code
            assert!(commands.is_empty());
        }

        #[test]
        fn test_generate_arc_commands_full_code() {
            let code = 255; // All bits set (8 bits)
            let bits = 8;
            let center = (100.0, 100.0);
            let radius = 50.0;

            let commands = generate_arc_commands(code, bits, center, radius);

            // Should generate commands for all segments
            assert!(!commands.is_empty());
            let command_count = commands.matches("path").count();
            assert_eq!(command_count, 8);
        }
    }

    mod integration {
        use super::*;

        #[test]
        fn test_full_code_generation_pipeline() {
            // Generate codes
            let codes = generate_codes(6, None, Some(3));

            // Verify all codes meet requirements
            for &code in &codes {
                // Even parity
                assert!(calc_parity(code));

                // Check opposite segments requirement
                let half_bits = 6 >> 1;
                let diff = (code & ((1 << half_bits) - 1))
                    & ((code & (((1 << half_bits) - 1) << half_bits)) >> half_bits);
                assert!(diff > 0);

                // Should be minimal rotation
                let minimal = find_smallest_rotation(code, 6);
                assert_eq!(code, minimal);
            }
        }

        #[test]
        fn test_geometric_calculations_consistency() {
            let center = (150.0, 150.0);
            let radius = 75.0;

            // Test that coordinates are consistent across different angles
            for angle in [0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0] {
                let coords = angle_to_coordinates(angle, radius, center);

                // Calculate distance from center
                let dx = coords.0 as f64 - center.0;
                let dy = coords.1 as f64 - center.1;
                let distance = (dx * dx + dy * dy).sqrt();

                // Should be approximately equal to radius (within 2 pixels due to integer conversion and floating point precision)
                assert!(
                    (distance - radius).abs() <= 2.0,
                    "Distance {} not close to radius {} for angle {}",
                    distance,
                    radius,
                    angle
                );
            }
        }
    }
}
