use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This time we have a set of value ranges, and another set of values. We need to figure out which values
doesn't fit inside any of the ranges.

The quick and easy solution would be to simply load the ranges into a "range" struct which holds the lower
and upper value and has a method "in_range" which takes a value and returns true/false depending on if the
value fits inside the range.

Simply a if statement: if value < lower || value > upper
Iterate through the list of ranges until the value fits in a range or all ranges have been checked.
If the value fits in a range then add 1 to a counter.



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
    let expected_value = 3;
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
