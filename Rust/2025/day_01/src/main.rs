use std::error::Error;

mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 3;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 1029;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 6;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################
The main task here is to figure out a way to have a number type that limits its values to 0..99
with overflow. Meaning subtraction past 0 will flow over as if 0 was 99. If we have 5 and subtract
10 then 5 would be removed from the subtraction value, 10 -> 5, and then we would need to subtract
100 with that remaining value. Meaning we get 95 as a result. Note that we subtract 100 not 99 as
100 is the same as 0 in our case. If we would subtract 99 we would "skip" 0, offsetting the result
by -1.

Update:
It is actually a lot simpler than that. We only really need to figure out how many times the dial
points at 0. But 0 doesn't actually have to be 0. 100 would also point at 0. So would -2300.
Basically, we can just add/subtract each rotation from the total. And after each operation just
check if the current value %100 is 0. If it is then add 1 to result.

*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let (mut result, mut dial_value) = (0, 50);
        for line in reader::get_lines(data_path)? {
            let v = &line[1..].parse::<i32>()?;
            match line.chars().next().ok_or("Unexpected empty line!")? {
                'R' => dial_value += v,
                'L' => dial_value -= v,
                _ => return Err("Invalid direction char!".into()),
            }
            result += if dial_value % 100 == 0 { 1 } else { 0 }
        }
        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################
This adds some difficulty in the form of keeping track of how many times we pass -1. The quick and ugly fix to
get it working was to:
0: Dial.turn() returns a "passed" value instead of the current dial position.
1: Add a counter in the Dial.turn() logic which:
    0: Divide the steps by 100 ignoring decimals. Add this value to passes.
    1: If direction is Left then increase passes by one if subtracting the steps from the current position would
       result in a value below -1 AND if the current dial position is NOT 0.
    2: If direction is Right then increase passes by one if adding the steps to the current position would result
       in a value higher than 98.
2: Edit the loop going through each line to always add the result of dial.turn() to the count variable.

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let (mut result, mut dial_value) = (0, 50i32);
        let mut section = (1, 0);
        for line in reader::get_lines(data_path)? {
            println!("dial: {} rotate by: {}", dial_value, line,);
            let v = &line[1..].parse::<u64>()?;
            result += v / 100;
            match line.chars().next().ok_or("Unexpected empty line!")? {
                'R' => dial_value += *v as i32 % 100,
                'L' => dial_value -= *v as i32 % 100,
                _ => return Err("Invalid direction char!".into()),
            }
            let cv = dial_value / 100;
            if dial_value % 100 == 0 {
                result += 1;
                section = (dial_value / 100, (dial_value / 100) - 1);
            } else if i32::min((section.0 - cv).abs(), (section.1 - cv).abs()) != 0 {
                result += 1;
            } else {
                section = (dial_value / 100, dial_value / 100);
            }

            //result += (last_val - cv).unsigned_abs() as u64;
            //last_val = cv;
            println!("result: {}", result);
        }
        Ok(result)
    }
}

/*
fn load_data(path: &str) -> Result<(), Box<dyn Error>> {
    let lines = reader::get_lines(path)?;

    todo!();
} // */

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;
    let mut dial = Dial::new();
    let mut count = 0;
    for line in lines {
        let rotation = Rotation::parse(&line)?;
        count += dial.turn(rotation);
    }

    todo!();

    Ok(count)
}

#[derive(Debug)]
struct Rotation {
    steps: i16,
}

impl Rotation {
    fn parse(instruction: &str) -> Result<Rotation, Box<dyn Error>> {
        let direction = match &instruction[0..1] {
            "L" => -1,
            "R" => 1,
            _ => {
                return Err(
                    format!("Failed to parse direction of rotation: {}", instruction).into(),
                )
            }
        };

        let steps = match instruction[1..].parse::<i16>() {
            Ok(steps) => steps,
            Err(e) => {
                return Err(format!(
                    "Failed to parse steps of rotation: {}\nError: {}",
                    instruction, e
                )
                .into())
            }
        };

        Ok(Rotation {
            steps: steps * direction,
        })
    }
}

#[derive(Debug)]
struct Dial {
    position: i16,
}

impl Dial {
    fn new() -> Dial {
        Dial { position: 50 }
    }

    fn turn(&mut self, rotation: Rotation) -> u64 {
        let mut passes = (rotation.steps.abs() / 100) as u64;

        // It wasn't clear at first glance that the rotations could be more than 99 steps, but as
        // the data file does contain such values we use this to basically cut away the excess full
        // rotations and only keep the part that matters.
        let steps = rotation.steps % 100;

        self.position += steps;

        // The issue now is that it counts as a pass when it starts at 0
        if self.position > 99 {
            self.position -= 100;
            passes += 1;
        } else if self.position < 0 {
            self.position += 100;
            passes += 1;
        }

        /*
        match rotation.direction {
            Dir::Left => {
                if self.position != 0 && self.position <= steps {
                    passes += 1;
                }
                if self.position < steps {
                    self.position = self.position + 100 - steps;
                } else {
                    self.position -= steps;
                }
            }
            Dir::Right => {
                self.position += steps;
                if self.position > 99 {
                    self.position -= 100;
                    passes += 1;
                }
            }
        } */

        // Debug output
        if passes != 0 {
            println!(
                "{:?} caused the dial to point at [0] {} times and stopped at the position: {}",
                rotation, passes, self.position
            );
        } // */
        passes
    }
}

/*
#[test]
fn calculate_test() {
    let expected_value = 6;
    match calculate("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}*/

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
