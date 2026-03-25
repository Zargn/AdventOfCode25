#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 142;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 55971;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

The task here is to simply grab the first and the last digit of each line and adding them
together. The first digit has to be multiplied by 10. (If only one digit exists then it is both
first and last.)
Then add the resulting values of each line together to get the result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn get_calibration_value(data_str: &str) -> Result<u64, Box<dyn Error>> {
        // for each char in data_str filter out any that are not ascii digits, then parse a int
        // from each digit and collect into a vec.
        let values: Vec<u32> = data_str
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap()) // c is always a digit so this will never panic.
            .collect();

        if values.is_empty() {
            Err("No values found!".into())
        } else {
            // values will always contain at least one element so this will never panic.
            Ok(((values.first().unwrap() * 10) + values.last().unwrap()) as u64)
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut result_value = 0;
        for line in reader::get_lines(data_path)? {
            result_value += get_calibration_value(&line)?;
        }

        Ok(result_value)
    }
}

//

//

/*
Part Two
##################################################################################################

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");

    if cfg!(feature = "bench") {
        println!("Benchmarks are enabled!\n");
    }

    println!("\nPart One {}\n", {
        match benchmark!("calculate", { part_one::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
    println!("\nPart Two {}\n", {
        match benchmark!("calculate", { part_two::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
}
