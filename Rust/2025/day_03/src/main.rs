use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

We have a list of numbers where we need to select any two numbers in the order of left to
right that forms the largest two digit number when combined as a string.

One way to do this is to:
Load each digit into a list.
Iterate through said list until the second last number.
    - If the number is larger than the largest yet then save it as the first highest
      value and clear the second highest value.
    - Else if the number is larger than the second highest then save the number as the
      second highest value.
After the loop check if the final number is larger than the second highest. If yes then
replace the second highest with the last number.
Return (first highest value * 10) + second highest value.

Improvement idea:
Instead of comparing the battery in the else if statement we would:
    - Replace else if with just if.
    - Replace the battery with the battery after the current one.
This would allow us to skip some of the extra stuff after the loop.



Part Two:

We still have a list of numbers but now we need to select 12 digits instead of 2 with the
same rules as before.

The first idea for this is to:
Keep a count of how many digits is left to select.
Have a list of (digit, index) values.
Have highest_value, value_index and index variables.
In a while loop:
    - If index >= batteries in the battery bank - digits left to select:
        - Set index to value_index + 1.
        - Add highest_value and value_index to the list and clear the old values.
        - Subtract 1 from digits left to select.
        If digits left to select <= 0:
            - Break loop.

    - Get the battery at index.
    - If the battery is higher then highest_value:
        - Set highest value to battery.
        - Set value_index to index.

    Add 1 to index.
return the list of digits assembled into a full 12 digit integer.



*/

struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    fn parse(data_string: &str) -> Result<BatteryBank, Box<dyn Error>> {
        let mut batteries = Vec::new();
        for c in data_string.chars() {
            batteries.push(c.to_string().parse()?);
        }
        Ok(BatteryBank { batteries })
    }

    fn joltage(&self) -> u8 {
        let (mut highest, mut second_highest) = (0, 0);
        for i in 0..self.batteries.len() - 1 {
            let battery = self.batteries[i];
            if battery > highest {
                highest = battery;
                second_highest = 0;
            }
            let next_battery = self.batteries[i + 1];
            if next_battery > second_highest {
                second_highest = next_battery;
            }
        }
        (highest * 10) + second_highest
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut count: u64 = 0;
    for line in reader::get_lines(data_path)? {
        count += BatteryBank::parse(&line)?.joltage() as u64;
    }

    Ok(count)
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 3121910778619;
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
