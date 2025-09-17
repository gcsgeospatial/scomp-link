use scomp_link::*;

#[test]
fn compare_with_python_output() {
    println!("Rust implementation test results:");

    // Test bitwise rotation
    let result = bitwise_rotate_left(176, 2, 8);
    println!("bitwise_rotate_left(176, 2, 8) = {}", result);
    assert_eq!(result, 194); // Expected from Python

    // Test smallest rotation
    let result = find_smallest_rotation(2816, 12);
    println!("find_smallest_rotation(2816, 12) = {}", result);
    assert_eq!(result, 11); // Expected from Python

    // Test parity
    let result = calc_parity(3);
    println!("calc_parity(3) = {}", result);
    assert_eq!(result, true); // Expected from Python

    let result = calc_parity(7);
    println!("calc_parity(7) = {}", result);
    assert_eq!(result, false); // Expected from Python

    // Test bit transitions
    let result = count_bit_transitions(5);
    println!("count_bit_transitions(5) = {}", result);
    assert_eq!(result, 2); // Expected from Python

    // Test code generation
    let codes = generate_codes(6, None, Some(3));
    println!("generate_codes(6, None, 3) = {:?}", codes);
    assert_eq!(codes, vec![9, 15, 23]); // Expected from Python

    // Test with transitions
    let codes = generate_codes(8, Some(2), Some(3));
    println!("generate_codes(8, 2, 3) = {:?}", codes);
    assert_eq!(codes, vec![17, 23, 27]); // Expected from Python

    // Test coordinates
    let coords = angle_to_coordinates(0.0, 50.0, (100.0, 100.0));
    println!(
        "angle_to_coordinates(0.0, 50.0, (100.0, 100.0)) = {:?}",
        coords
    );
    assert_eq!(coords, (100, 50)); // Expected from Python
}
