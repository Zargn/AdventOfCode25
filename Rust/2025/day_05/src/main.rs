mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 3;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 598;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 14;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 360341832208407;

//

//

/*
Part One
##################################################################################################

This time we have a set of value ranges, and another set of values. We need to figure out which values
doesn't fit inside any of the ranges.

The quick and easy solution would be to simply load the ranges into a "range" struct which holds the lower
and upper value and has a method "in_range" which takes a value and returns true/false depending on if the
value fits inside the range.

Simply a if statement: if value >= lower && value <= upper
Iterate through the list of ranges until the value fits in a range or all ranges have been checked.
If the value fits in a range then add 1 to a counter.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

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

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
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
}

//

//

/*
Part Two
##################################################################################################

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
mod part_two {
    use crate::reader;
    use std::error::Error;

    #[derive(Clone, Copy, Debug)]
    struct IDRange {
        upper: u64,
        lower: u64,
    }

    impl IDRange {
        fn in_range(&self, value: &u64) -> bool {
            self.lower <= *value && *value <= self.upper
        }

        fn attempt_merge(&mut self, other: &IDRange) -> bool {
            if !((self.upper >= other.lower && self.lower <= other.lower)
                || (self.upper >= other.upper && self.lower <= other.upper)
                || (self.upper <= other.upper && self.lower >= other.lower))
            {
                return false;
            }

            let new_upper = if self.upper >= other.upper {
                self.upper
            } else {
                other.upper
            };
            let new_lower = if self.lower <= other.lower {
                self.lower
            } else {
                other.lower
            };

            self.upper = new_upper;
            self.lower = new_lower;

            true
        }

        fn range_count(&self) -> u64 {
            self.upper - self.lower + 1 // Add 1 since the range is inclusive.
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

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines: Vec<String> = reader::get_lines(data_path)?.collect();
        let mut ranges: Vec<IDRange> = Vec::new();

        for line in lines.iter().take_while(|line| !line.is_empty()) {
            ranges.push(IDRange::compressed_parse(line)?);
        }

        let mut ranges_len = ranges.len();
        let mut outer_index = 0;
        while outer_index < ranges_len {
            let mut range = ranges[outer_index];
            let mut i = outer_index + 1;
            while i < ranges_len {
                let other_range = ranges[i];
                if range.attempt_merge(&other_range) {
                    ranges.remove(i);
                    ranges[outer_index] = range;
                    i = outer_index + 1;
                    ranges_len -= 1;
                } else {
                    i += 1;
                }
            }
            outer_index += 1;
        }

        let mut count = 0;
        for range in ranges {
            count += range.range_count();
        }

        Ok(count)
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    print!("Running Program...\n\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}\n", value),
        Err(err) => println!("FAILED with error:\n{}\n", err),
    }
}
