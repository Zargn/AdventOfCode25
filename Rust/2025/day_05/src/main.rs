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

At first glance part two seems simple. We just need to add together the ranges in the data file and count
how many valid IDs can exist.
But the issue is that some ranges overlap. So we need to somehow detect that and prevent counting the same
id twice.

One solution to this is to simple count all of them and place each value in a hashset to track if it has
been counted before. But the memory usage for this would be huge, and the performance would likely also
suffer.

Another option would be to combine the ranges before counting them.
Basically iterate through the ranges checking if they overlap. If two overlap then remove them both and
create a new one with their ranges combined.

This could be done by:

for range in ranges
    let i = 0
    while i < ranges.len()
        other_range = ranges[i]
        if other_range overlaps with range
            add other_range to range
            remove other_range from the list
            i = 0 // Reset loop to check if earlier ranges overlap with the new range

This should be enough to merge any overlapping ranges.

Then count the total by looping through all ranges adding the following to the total
range.upper - range.lower



*/

struct IDRange {
    upper: u64,
    lower: u64,
}

impl IDRange {
    fn in_range(&self, value: &u64) -> bool {
        self.lower <= *value && *value <= self.upper
    }

    fn parse(data_string: &str) -> Result<IDRange, Box<dyn Error>> {
        let mut parts = data_string.split('-');
        let (Some(lower), Some(upper)) = (parts.next(), parts.next()) else {
            return Err("Invalid Range Format".into());
        };
        Ok(IDRange {
            upper: upper.parse()?,
            lower: lower.parse()?,
        })
    }

    /// Compressed version of parse().
    /// Does the same thing but with less code.
    fn compressed_parse(data_string: &str) -> Result<IDRange, Box<dyn Error>> {
        let mut parts = data_string.split('-');
        Ok(IDRange {
            lower: parts.next().ok_or("Invalid Range Format")?.parse()?,
            upper: parts.next().ok_or("Invalid Range Format")?.parse()?,
        })
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

/// Compressed version of calculate.
/// Does the same thing as calculate() but with more compact code. Sacrifices some error clarity.
fn compressed_calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines: Vec<String> = reader::get_lines(data_path)?.collect();
    let mut ranges: Vec<IDRange> = Vec::new();

    for line in lines.iter().take_while(|line| !line.is_empty()) {
        ranges.push(IDRange::compressed_parse(line)?);
    }

    let mut fresh_ingredients: u64 = 0;
    for line in lines.iter().skip(ranges.len() + 1) {
        let value: u64 = line.parse()?;
        if ranges.iter().any(|range| range.in_range(&value)) {
            fresh_ingredients += 1;
        }
    }

    Ok(fresh_ingredients)
}

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    todo!();
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
    match compressed_calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
    println!("Part Two:");
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

#[test]
fn calculate_part_two_test() {
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

#[test]
fn compressed_calculate_test() {
    let expected_value = 3;
    match compressed_calculate("testdata.txt") {
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
