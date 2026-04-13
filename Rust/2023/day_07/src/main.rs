#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 6440;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Time for some playing cards.

The data consists of rows where each row contains a hand and a bid. Our goal is to calculate the
total winnings of all hands.

We get the winnings of each row by multiplying the bid with the rank of the hand compared to all
others. Meaning the least valuable hand has rank 1, and the most valuable has a rank equal to the
amount of data rows (I.e. hands) in the data.

Value is determined using these rules:

Each hand has 5 characters with values in the following range ordered from most to least value.
A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2

Each hand has one of the following patterns again ordered from most to least valuable.
Five of a kind: 5X1
Four of a kind: 4x1 1x1
Full house: 3x1 2x1
Three of a kind: 3x1 1x1 1x1
Two pair: 2x1 2x1 1x1
One pair: 2x1 1x1 1x1
High card: five different cards.

The rank of two hands with the same pattern is decided by comparing characters left to right.
The first hand to have a higher value character during comparison gets the higher rank.



So, we need to read each hand order them in a list based on their value.
We could do this two ways. Either we calculate the rank of each hand as we read it, updating
the order as we go.
Or we read each hand recording it's pattern and characters, then placing each processed hand
in a list. We can then create our own ordering implementation where we do the above checks to
order them. Then we can use a library sorting algortihm to automatically sort the list.

Once we have our sorted list of hands we can just go through them multiplying their bids with
their rank, then adding it to a sum to get the result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
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
