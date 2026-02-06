#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 4277556;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 3525371263915;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 3263827;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 6846480843636;

//

//

/*
Part One
##################################################################################################

This is an interesting task. We need to read and then perform multiplication/addition with 3 values.
It would be "easy" to do if each group of 3 values and operator was on separate rows. But here the
first value is on the same row as all other groups first values. The second is on the same row as
all other second values. And so on.

So we need to read the data one column at a time instead of row.
I think the quickes solution is to read the data and collect it into groups first, instead of trying
to process only one column at a time.
This should be quite easy using string.split(" ") and filter.
*/
mod part_one {
    use crate::reader;
    use std::{
        error::Error,
        fmt::{write, Display},
    };

    enum Operator {
        Add,
        Mul,
    }

    impl Operator {
        fn parse(data_string: &str) -> Result<Operator, Box<dyn Error>> {
            match data_string {
                "+" => Ok(Operator::Add),
                "*" => Ok(Operator::Mul),
                _ => Err(format!("Could not parse operator from: [{}]", data_string).into()),
            }
        }

        fn use_on(&self, a: &u64, b: &u64) -> u64 {
            match self {
                Self::Add => *a + *b,
                Self::Mul => *a * *b,
            }
        }

        fn is_mul(&self) -> bool {
            matches!(self, Self::Mul)
        }
    }

    fn is_not_empty(str: &&str) -> bool {
        !str.is_empty()
    }

    #[derive(Debug)]
    enum PuzzleError {
        DifferentLineLenght,
    }

    impl Display for PuzzleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Puzzle Error: {}",
                match self {
                    Self::DifferentLineLenght => "DifferentLineLenght",
                }
            )
        }
    }

    impl Error for PuzzleError {}

    pub fn calculate(path: &str) -> Result<u64, Box<dyn Error>> {
        let lines: Vec<String> = reader::get_lines(path)?.collect();

        let mut value_lines: Vec<Vec<u64>> = Vec::new();
        for line in lines.iter().take(lines.len() - 1) {
            let mut values = Vec::new();
            for value_str in line.split(' ').filter(is_not_empty) {
                values.push(value_str.parse()?);
            }
            value_lines.push(values);
        }

        let mut operators: Vec<Operator> = Vec::new();
        for operator_str in lines[lines.len() - 1].split(' ').filter(is_not_empty) {
            operators.push(Operator::parse(operator_str)?);
        }

        if value_lines.iter().any(|line| line.len() != operators.len()) {
            //return Err("Data file contains lines with different amount of columns!".into());
            return Err(Box::new(PuzzleError::DifferentLineLenght));
        }

        let mut sum = 0;
        for (i, operator) in operators.iter().enumerate() {
            let mut line_sum = if operator.is_mul() { 1 } else { 0 };
            for value in value_lines.iter().map(|line| line[i]) {
                line_sum = operator.use_on(&line_sum, &value);
            }
            sum += line_sum;
        }

        Ok(sum)
    }
}

//

//

/*
Part Two
##################################################################################################

This is pure evil.
Now we need to read the values one column at a time.
It seems the length of the numbers vary but the operator signals the index where the columns start.
There is still one extra space between. This can be ignored since it should always be empty.
So:
+   +
01234
Means that the first value starts at index 0 and ends at index 2 (Inclusive)

I think the solution here could be to:
Load all lines into lists of chars.
Have a sum, temp_value and operator variable
Iterate i from 0 to the length of the lists
    Create a number by combining index i in all the number lists
    if the char at index i in the operators list is a operator then:
        Add temp_value to sum and clear the old temp_value
        Set the operator variable to the operator we found

    increase temp_value by using the operator on temp_value with the number
Once the loop is done add temp_value to sum and return the result
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(path: &str) -> Result<u64, Box<dyn Error>> {
        let lines: Vec<Vec<char>> = reader::get_lines(path)?
            .map(|line| line.chars().collect())
            .collect();

        let line_length = lines[0].len();
        if lines.iter().any(|line| line.len() != line_length) {
            println!("{:?}", lines);
            return Err("The data lines does not have the same lenth!".into());
        }

        let (mut sum, mut operator) = (0, Operator::Add(0));
        for i in 0..lines[0].len() {
            if let Ok(new_operator) = Operator::parse(&lines[lines.len() - 1][i].to_string()) {
                sum += operator.value();
                println!("Value: {}", operator.value());
                operator = new_operator;
            }

            if let Ok(value) = {
                let mut s = String::new();
                for line in lines.iter().take(lines.len() - 1) {
                    if line[i] != ' ' {
                        s.push(line[i]);
                    }
                }
                s.parse::<u16>()
            } {
                operator.combine(&value);
                //print!(" {value} ");
            }
        }
        sum += operator.value();

        Ok(sum)
    }

    enum Operator {
        Add(u64),
        Mul(u64),
    }

    impl Operator {
        fn parse(data_string: &str) -> Result<Operator, Box<dyn Error>> {
            match data_string {
                "+" => Ok(Operator::Add(0)),
                "*" => Ok(Operator::Mul(1)), // 1 since the first combine would otherwise multiply by 0
                _ => Err(format!("Could not parse operator from: [{}]", data_string).into()),
            }
        }

        fn combine(&mut self, other_value: &u16) {
            match self {
                Self::Add(value) => *value += *other_value as u64,
                Self::Mul(value) => *value *= *other_value as u64,
            }
        }

        fn value(&self) -> u64 {
            match self {
                Self::Add(value) => *value,
                Self::Mul(value) => *value,
            }
        }
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
    println!("\nPart One {}\n", {
        match benchmark!("calculate", { part_two::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
}
