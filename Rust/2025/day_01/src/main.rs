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
pub const PART_TWO_EXPECTED_VALUE: u64 = 5892;

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

Update: The above is no longer used. A alternate solution was created.

*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let (mut result, mut dial_value, mut at_zero) = (0, 50, false);
        for line in reader::get_lines(data_path)? {
            let rotation = &line[1..].parse::<u64>()?;
            result += rotation / 100;
            match line.chars().next().ok_or("Unexpected empty line!")? {
                'R' => dial_value += *rotation as i32 % 100,
                'L' => dial_value -= *rotation as i32 % 100,
                _ => return Err("Invalid direction char!".into()),
            }

            if !(1..=99).contains(&dial_value) && !at_zero {
                result += 1;
            }

            // Produces the same result as +/- 100 to ensure it is within 0-99
            dial_value = dial_value.rem_euclid(100);

            at_zero = dial_value == 0;
        }
        Ok(result)
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
