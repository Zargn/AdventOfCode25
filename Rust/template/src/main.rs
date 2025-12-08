use std::error::Error;

mod reader;

#[allow(dead_code)]
const PART_ONE_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;

/*
Part One
##################################################################################################

*/

fn calculate_part_one(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;

    Err("Not implemented!".into())
}

/*
Part Two
##################################################################################################

*/

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;

    Err("Not implemented!".into())
}

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Part One Result: ");
    match calculate_part_one("data.txt") {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error: {}", err),
    }
    println!("\nPart Two Result: ");
    match calculate_part_two("data.txt") {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error: {}", err),
    }
}

#[test]
fn calculate_part_one_test() {
    let expected_value = PART_ONE_EXPECTED_TEST_VALUE;
    match calculate_part_one("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part One calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part One Error:\n{}", err),
    }
}

#[test]
fn calculate_part_two_test() {
    let expected_value = PART_TWO_EXPECTED_TEST_VALUE;
    match calculate_part_two("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part Two calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part Two Error:\n{}", err),
    }
}
