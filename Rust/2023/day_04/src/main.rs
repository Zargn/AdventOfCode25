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
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 30;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 5132675;

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

This will be a bit more complex. We still use the same logic to see how many "wins" each card
have. But how we handle those wins differ.

This time the matches are used to "duplicate" later cards. Basically, if a card has two matches,
then the next two cards below this one are duplicated. If only one, then only the card below is.

So, we can use the same code as part one but tweak the process card function to return how many
winning numbers that it has.

One possible solution is the following:
In the main function we collect all the process_card() return values (Including zeros) and place
them in a list together with a second integer at each index. Vec<(u64, u64)> where the first u64
is the return value of process_card for that row, and the second value is a 1.

Once the list is completed we go back and iterate through it from the start.
for i in 0..list.len {
    let (matches, count) = list[i];

    // Then we add [count] to the next [matches] elements [count] value.
    for i2 in 1..matches {
        list[i+i2].count += count.
    }
}

Once we reach the end go back and sum all the [count] values in the list to get the answer.

The goal is to figure out how many cards we end up with at the end.
*/
mod part_two {
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

        let matches = extract_integers(our_numbers)?
            .iter()
            .filter(|nr| winning_numbers.contains(nr))
            .count();

        Ok(matches as u64)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        // Reads all the lines of the datafile, checks how many matches each card has, then
        // collecting them all into a list of value pairs (matches, 1) for each row.
        let mut cards: Vec<(u64, u64)> = reader::get_lines(data_path)?
            .map(|line| process_card(&line).map(|v| (v, 1)))
            .collect::<Result<Vec<(u64, u64)>, _>>()?;

        // Iterates through the list calculating the count of each card.
        for i in 0..cards.len() {
            let (matches, count) = cards[i];
            for i in i + 1..i + 1 + matches as usize {
                if let Some((_, other_count)) = cards.get_mut(i) {
                    *other_count += count;
                }
            }
        }

        // Sums all the card counts in the list.
        Ok(cards.iter().map(|(_, count)| count).sum())
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
