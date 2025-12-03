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

struct IDRange {
    lower: u32,
    upper: u32,
}

impl IDRange {
    fn parse(data_string: &str) -> Result<IDRange, Box<dyn Error>> {
        let mut parts = data_string.split('-');
        let lower = parts.next();
        let upper = parts.next();
        let (Some(lower), Some(upper)) = (lower, upper) else {
            return Err(format!(
                "Could not parse IDRange from data string: [{}]!",
                data_string
            )
            .into());
        };
        Ok(IDRange {
            lower: lower.parse()?,
            upper: upper.parse()?,
        })
    }

    fn invalid_id_sum(&self) -> u64 {
        let mut id_sum = 0;
        for id in self.lower..self.upper {
            let id_string = id.to_string();
            let id_string_len = id_string.chars().count();
            if (id_string_len % 2) != 0 {
                continue;
            }
            if id_string[0..id_string_len / 2] == id_string[id_string_len / 2..] {
                id_sum += id as u64;
            }
        }
        id_sum
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let line = reader::get_lines(data_path)?
        .next()
        .expect("The data files for this challenge always only contain 1 line.");
    let mut id_sum = 0;
    for data_string in line.split(',') {
        let id_range = IDRange::parse(data_string)?;
        id_sum += id_range.invalid_id_sum();
    }

    Ok(id_sum)
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
