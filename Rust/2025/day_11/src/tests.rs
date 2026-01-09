use crate::{
    part_one, part_two, PART_ONE_EXPECTED_TEST_VALUE, PART_ONE_EXPECTED_VALUE,
    PART_TWO_EXPECTED_TEST_VALUE, PART_TWO_EXPECTED_VALUE,
};

/// Checks if the provided expected_value has been set, returning it if it has.
fn test_configured<T: Default + PartialEq>(expected_value: T) -> Option<T> {
    match expected_value {
        value if value != T::default() => Some(value),
        _ => None,
    }
}

#[test]
fn calculate_part_one_example() {
    let Some(expected_value) = test_configured(PART_ONE_EXPECTED_TEST_VALUE) else {
        return;
    };

    match part_one::calculate("testdata.txt") {
            Ok(value) => assert_eq!(
                value, expected_value,
                "Part One calculation completed successfully but the result was wrong! Expected: {} but received: {}",
                expected_value, value
            ),
            Err(err) => panic!("Part One failed with error:\n{}\n", err),
        }
}

#[test]
fn calculate_part_one_full() {
    let Some(expected_value) = test_configured(PART_ONE_EXPECTED_VALUE) else {
        return;
    };
    match part_one::calculate("data.txt") {
            Ok(value) => assert_eq!(
                value, expected_value,
                "Part One calculation completed successfully but the result was wrong! Expected: {} but received: {}",
                expected_value, value
            ),
            Err(err) => panic!("Part One failed with error:\n{}\n", err),
        }
}

#[test]
fn calculate_part_two_example() {
    let Some(expected_value) = test_configured(PART_TWO_EXPECTED_TEST_VALUE) else {
        return;
    };
    match part_two::calculate("testdata_two.txt") {
            Ok(value) => assert_eq!(
                value, expected_value,
                "Part Two calculation completed successfully but the result was wrong! Expected: {} but received: {}",
                expected_value, value
            ),
            Err(err) => panic!("Part Two failed with error:\n{}\n", err),
        }
}

#[test]
fn calculate_part_two_full() {
    let Some(expected_value) = test_configured(PART_TWO_EXPECTED_VALUE) else {
        return;
    };
    match part_two::calculate("data.txt") {
            Ok(value) => assert_eq!(
                value, expected_value,
                "Part Two calculation completed successfully but the result was wrong! Expected: {} but received: {}",
                expected_value, value
            ),
            Err(err) => panic!("Part Two failed with error:\n{}\n", err),
        }
}
