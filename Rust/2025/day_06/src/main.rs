use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This is an interesting task. We need to read and then perform multiplication/addition with 3 values.
It would be "easy" to do if each group of 3 values and operator was on separate rows. But here the
first value is on the same row as all other groups first values. The second is on the same row as
all other second values. And so on.

So we need to read the data one column at a time instead of row.
I think the quickes solution is to read the data and collect it into groups first, instead of trying
to process only one column at a time.
This should be quite easy using string.split(" ") and filter.



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
    let expected_value = 4277556;
    match calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

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
