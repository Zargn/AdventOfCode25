use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*

Part One:

The main task here is to figure out a way to have a number type that limits its values to 0..99 with overflow.
Meaning subtraction past 0 will flow over as if 0 was 99. If we have 5 and subtract 10 then 5 would be removed
from the subtraction value, 10 -> 5, and then we would need to subtract 100 with that remaining value.
Meaning we get 95 as a result. Note that we subtract 100 not 99 as 100 is the same as 0 in our case. If we
would subtract 99 we would "skip" 0, offsetting the result by -1.



Part Two:

*/

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
        let current_position = dial.turn(rotation);
        if current_position == 0 {
            count += 1;
        }
    }

    Ok(count)
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

enum Dir {
    Left,
    Right,
}

struct Rotation {
    direction: Dir,
    steps: u16,
}

impl Rotation {
    fn parse(instruction: &str) -> Result<Rotation, Box<dyn Error>> {
        let direction = match &instruction[0..1] {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => {
                return Err(
                    format!("Failed to parse direction of rotation: {}", instruction).into(),
                )
            }
        };

        let steps = match instruction[1..].parse() {
            Ok(steps) => steps,
            Err(e) => {
                return Err(format!(
                    "Failed to parse steps of rotation: {}\nError: {}",
                    instruction, e
                )
                .into())
            }
        };
        Ok(Rotation { direction, steps })
    }
}

struct Dial {
    position: u8,
}

impl Dial {
    fn new() -> Dial {
        Dial { position: 50 }
    }

    fn turn(&mut self, rotation: Rotation) -> u8 {
        // It wasn't clear at first glance that the rotations could be more than 99 steps, but as
        // the data file does contain such values we use this to basically cut away the excess full
        // rotations and only keep the part that matters.
        let steps = (rotation.steps % 100) as u8;

        match rotation.direction {
            Dir::Left => {
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
                }
            }
        }
        self.position
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
