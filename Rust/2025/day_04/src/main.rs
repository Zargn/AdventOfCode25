use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This time we have a grid of either empty or occupied tiles.
We need to figure out which occupied tiles have at most 4 other occupied tiles in the surrounding 8 tiles.

We need to build a two dimensional array of tiles first.
Then iterate thorugh all tiles.
For each tile we chack the surrounding 8 for other occupied tiles and count them.
If there are at most 3 other occupied tiles then add 1 to a counter.

Once all tiles have been visited then return the count value.

Improvement:
If we give the two dimensional array 1 layer of empty tils all around the data tiles then we won't need to
check that the tile being checked is within the array.



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
    let expected_value = 13;
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
