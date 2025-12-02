use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

First step here is to read the string and convert it to a sequnce of number pairs.
Once that is done we can process each number in the range of the pair.

Processing could work like this:
Convert the number into a string.
If the string doesn't have an even number of digits then it is guaranteed to be valid.
Split the string in half. If both parts are the same then the ID is invalid.



Part Two:



*/

fn load_data(path: &str) -> Result<(), Box<dyn Error>> {
    let lines = reader::get_lines(path)?;

    todo!();
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let data = load_data(data_path)?;
    todo!();
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 1227775554;
    match calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

/*
#[test]
fn calculate_small_test() {
    let expected_value = 0;
    match calculate("smalltestdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using smalltestdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
} // */
