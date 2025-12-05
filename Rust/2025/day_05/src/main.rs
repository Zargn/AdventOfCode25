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

Simply a if statement: if value >= lower && value <= upper
Iterate through the list of ranges until the value fits in a range or all ranges have been checked.
If the value fits in a range then add 1 to a counter.



Part Two:



*/

struct IDRange {
    upper: u64,
    lower: u64,
}

impl IDRange {
    fn parse(data_string: &str) -> Result<IDRange, Box<dyn Error>> {
        let mut parts = data_string.split('-');
        let (Some(lower), Some(upper)) = (parts.next(), parts.next()) else {
            return Err(format!(
                "Could not parse IDRange from data string: [{}]!",
                data_string
            )
            .into());
        };
        Ok(IDRange {
            upper: upper.parse()?,
            lower: lower.parse()?,
        })
    }

    fn in_range(&self, value: &u64) -> bool {
        self.lower <= *value && *value <= self.upper
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut lines = reader::get_lines(data_path)?;
    let mut ranges: Vec<IDRange> = Vec::new();

    loop {
        let Some(line) = lines.next() else {
            return Err("Data file ended before any values was reached!".into());
        };
        if line.is_empty() {
            break;
        }
        ranges.push(IDRange::parse(&line)?);
    }

    let mut fresh_ingredients: u64 = 0;
    // This iter starts where the above loop stopped reading at the empty space.
    for line in lines {
        let value: u64 = line.parse()?;
        for range in &ranges {
            if range.in_range(&value) {
                //println!("[{}] is in range [{}-{}]", value, range.lower, range.upper);
                fresh_ingredients += 1;
                break;
            }
        }
    }

    Ok(fresh_ingredients)
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
