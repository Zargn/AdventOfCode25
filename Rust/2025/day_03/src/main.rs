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



Part Two:



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
        let mut highest = 0;
        let mut second_highest = 0;

        for i in 0..self.batteries.len() - 1 {
            let battery = self.batteries[i];
            if battery > highest {
                highest = battery;
                second_highest = 0;
            } else if battery > second_highest {
                second_highest = battery;
            }
        }
        let last_battery = self.batteries[self.batteries.len() - 1];
        if last_battery > second_highest {
            second_highest = last_battery;
        }

        (highest * 10) + second_highest
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;
    let mut count: u64 = 0;
    for line in lines {
        let battery_bank = BatteryBank::parse(&line)?;
        let joltage = battery_bank.joltage();
        count += joltage as u64;
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
    let expected_value = 357;
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
