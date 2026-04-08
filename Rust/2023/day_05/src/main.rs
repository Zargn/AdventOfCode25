#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 35;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Okay so we essentially have a repeating pattern that we need to handle.

The first row of the data file contains all the seeds, with the following rows following a set
pattern. Each part starts with a row containing the word "map". Then the following rows each
contain 3 values where the: 1st value = Destination range start, 2nd value = Source range start
and the 3rd value = range length.

Each line represents how to transform values within a set range. Lets say we have the row:
"42 64 8"
This means that any value between 64 and 64+8 needs to be shifted down to the start point of 42.
To do this we can subtract the source range start with the destination range start. Giving us the
difference. 64 - 42 = 22
We then use difference to shift the matching number to the destination range.
Lets say we have the value 68. 68 is within the range of 64 and 64+8. Then we subtract the
difference and return the value. 68 - 22 = 46.

If we create a "range" struct that does the above calculation then we can create a "map" struct
that contains a list of these ranges. Then we simply check each range in the map if the value
matches to get the transformed value. And if there are no matches we simply return the value
unchanged.

Then just chain these together according to the data file.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
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
