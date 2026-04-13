#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 288;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 32076;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

The datafile has two parts.
The first row holds the duration of each race.
And the second row holds the record distance, I.e. the distance we need to beat.

Our goal is to calculate how many different durations we can hold the button that still wins us
the race.

What I am thinking is we start to figure out the shortest time we need to hold the button that
still results in a win, and then after we calculate the longest possible duration. Since any
duration between those two will also result in a win, we don't need to check them.
So we can figure out how many options we have by getting the difference between the shortest and
longest possible durations.

Then simply multiply all the resulting values together.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn get_values(possible_row: Option<String>) -> Result<Vec<u64>, Box<dyn Error>> {
        Ok(possible_row
            .ok_or("Missing data row!")?
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?)
    }

    fn is_winner(race_time: u64, distance: u64, hold_time: u64) -> bool {
        (race_time - hold_time) * hold_time > distance
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;
        let times = get_values(lines.next())?;
        let distances = get_values(lines.next())?;

        let mut result = 1;

        for (race_time, distance) in times.iter().zip(distances) {
            let (mut lower, mut higher) = (0, 0);

            // Find the lowest hold time that results in a new record.
            for hold_time in 1..*race_time {
                if is_winner(*race_time, distance, hold_time) {
                    lower = hold_time;
                    break;
                }
            }

            // Find the highest hold time that results in a new record.
            for hold_time in (1..*race_time).rev() {
                if is_winner(*race_time, distance, hold_time) {
                    higher = hold_time;
                    break;
                }
            }

            result *= higher - lower + 1;
        }

        Ok(result)
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
