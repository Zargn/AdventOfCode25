mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 1227775554;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 24747430309;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 4174379265;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 30962646823;

//

//

/*
Part One
##################################################################################################

First step here is to read the string and convert it to a sequnce of number pairs.
Once that is done we can process each number in the range of the pair.

Processing could work like this:
Convert the number into a string.
Split the string in half. If both parts are the same then the ID is invalid.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    struct IDRange {
        lower: u64,
        upper: u64,
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
                lower: lower.parse()?,
                upper: upper.parse()?,
            })
        }

        fn invalid_id_sum(&self) -> u64 {
            let mut id_sum = 0;

            for id in self.lower..=self.upper {
                let id_string = id.to_string();
                let id_len = id_string.chars().count();
                if id_string[0..id_len / 2] == id_string[id_len / 2..] {
                    id_sum += id;
                }
            }

            id_sum
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut id_sum = 0;

        // get_lines returns an iterator over the lines of the file. next() attempts to return the
        // first line, which we then ensure is there with expect().
        for id_range_string in reader::get_lines(data_path)?
            .next()
            .expect("The data files for this challenge always only contain 1 line.")
            .split(',')
        {
            let id_range = IDRange::parse(id_range_string)?;
            id_sum += id_range.invalid_id_sum();
        }

        Ok(id_sum)
    }
}

//

//

/*
Part Two
##################################################################################################

Part two adds a quite annoying new requirement. Any id with a repeating pattern is invalid as long as it repeats at least once.
The first glance solution I can think of is to check each combination, which will be very slow.

Which would mean checking each pattern from the first digit to half of all digits.
So basically:
Take the first digit and iterate through the string. If at any point a different digit is found then move on to the next.
Take the first two digits and iterate through the string two steps at a time. If at any point the two digits checked doesn't
    match the first then continue to the next.
Repeat this up to the string length/2.
If any iterator completes without a mismatch then the id is invalid.

Unfinished random ideas.
A major point to optimise would be to ignore sequence lengths that would never occur in the range.
Check how similar the lower and upper values are left to right.
    235 - 278 only match with the first digit [2].
    53437 - 53495 match with the first three digits [534]
This can be used to skip checking some patterns. With the second range with the shared first digits of 534 we can figure out that:
- The 1 digit pattern is impossible due to the second digit [3] not matching the first [5].
- The 2 digit pattern is impossible due to the third digit [4] not matching the first [5].
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    struct IDRange {
        lower: u64,
        upper: u64,
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
                lower: lower.parse()?,
                upper: upper.parse()?,
            })
        }

        fn invalid_id_sum(&self) -> u64 {
            let mut id_sum = 0;

            for id in self.lower..=self.upper {
                let digit_count = id.ilog10() + 1;
                if Self::is_invalid(&id.to_string(), digit_count as usize) {
                    id_sum += id;
                }
            }

            id_sum
        }

        fn is_invalid(id_string: &str, digit_count: usize) -> bool {
            for pattern_len in 1..=digit_count / 2 {
                let pattern = &id_string[0..pattern_len];
                let mut i = pattern_len;
                let mut invalid = true;
                while i < digit_count {
                    if i + pattern_len > digit_count || pattern != &id_string[i..i + pattern_len] {
                        invalid = false;
                        break;
                    }
                    i += pattern_len;
                }
                if invalid {
                    return true;
                }
            }
            false
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut id_sum = 0;

        // get_lines returns an iterator over the lines of the file. next() attempts to return the
        // first line, which we then ensure is there with expect().
        for id_range_string in reader::get_lines(data_path)?
            .next()
            .expect("The data files for this challenge always only contain 1 line.")
            .split(',')
        {
            let id_range = IDRange::parse(id_range_string)?;
            id_sum += id_range.invalid_id_sum();
        }

        Ok(id_sum)
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
