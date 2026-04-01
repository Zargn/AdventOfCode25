#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 13;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 21959;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

This will be a "simple" compare values problem.
Each row of the data can be handled separately. The following will describe what to do with one
row.

The format is the following:
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
We only care about the normal numbers in the above string, but we need to group them correctly.
First, split the string at ':' AND '|'.
The first part "Card 1" can be ignored.
The second and third parts are the winning numbers and our numbers respectivly.

Next we use the same code to extract a list of u8 integers from the string. One way to do this
is to split the remaining string at any non-numberic character, while also filtering out empty
parts. This results in that any part returned by the split will always be a string that can be
parsed into an integer, as any non-digit character will have been filtered away.

When we get the two lists of values, we simply need to check how many matches there are. With the
first match we add 1 to the score, then each remaining matches doubles the score.
Once done return the score.

All that is needed after this is to read each line of the data, calling the above function on
each line. Then adding the scores together to get our puzzle answer.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    /// Extracts all integers in the provided string.
    ///
    /// Will return an error if a integer is too large to fit in a u8.
    fn extract_integers(str: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(str
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()?)
    }

    fn process_card(card_str: &str) -> Result<u64, Box<dyn Error>> {
        let mut parts = card_str.split(['|', ':']);
        parts.next();
        let (Some(winning_numbers), Some(our_numbers)) = (parts.next(), parts.next()) else {
            return Err(format!("Unexpected card string format! [{}]", card_str).into());
        };

        let winning_numbers = extract_integers(winning_numbers)?;
        let our_numbers = extract_integers(our_numbers)?;

        let score = benchmark!("Compare numbers: vec.contains()", {
            let mut score = 0;
            for value in &our_numbers {
                if winning_numbers.contains(value) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score = score + score;
                    }
                }
            }
            score
        });

        Ok(score)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut score = 0;
        for line in reader::get_lines(data_path)? {
            score += process_card(&line)?;
        }
        Ok(score)
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
