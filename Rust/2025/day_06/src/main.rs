use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This is an interesting task. We need to read and then perform multiplication/addition with 3 values.
It would be "easy" to do if each group of 3 values and operator was on separate rows. But here the
first value is on the same row as all other groups first values. The second is on the same row as
all other second values. And so on.

So we need to read the data one column at a time instead of row.
I think the quickes solution is to read the data and collect it into groups first, instead of trying
to process only one column at a time.
This should be quite easy using string.split(" ") and filter.



Part Two:

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

fn calculate(path: &str) -> Result<u64, Box<dyn Error>> {
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
            print!(" {value} ");
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

fn old_calculate(path: &str, value_lines_override: usize) -> Result<u64, Box<dyn Error>> {
    let mut lines = reader::get_lines(path)?;

    let mut value_lines: [Vec<u16>; 4] = [const { Vec::new() }; 4];
    for i in 0..value_lines_override {
        for value_string in lines
            .next()
            .ok_or("Datafile value lines does not match the provided value_lines_override value.")?
            .split(" ")
            .filter(|s| !s.is_empty())
        {
            value_lines[i].push(value_string.parse()?);
        }
    }

    let mut operators: Vec<Operator> = Vec::new();
    for operator_string in lines
        .next()
        .ok_or("There should always be a operators line after the values.")?
        .split(" ")
        .filter(|s| !s.is_empty())
    {
        operators.push(Operator::parse(operator_string)?);
    }

    for i in 0..value_lines_override {
        if value_lines[i].len() != operators.len() {
            return Err(
                "Data file contains lines with different amount of columns! Aborting...".into(),
            );
        }
    }

    println!("Len: {}", operators.len());

    let mut sum: u64 = 0;
    for (i, operator) in operators.iter_mut().enumerate() {
        for y in 0..value_lines_override {
            let other_value = &value_lines[y][i];
            print!(" {other_value} ");
            operator.combine(other_value);
        }
        let value = operator.value();
        println!("Value: {value}");
        sum += operator.value();
    }

    Ok(sum)
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 3263827;
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
