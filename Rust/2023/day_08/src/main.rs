#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 6;
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

Time for some map traversal again.

Our data constists of two parts.
The first row contains a pattern of instructions, and the following rows contain information about
each node and which other nodes are connected on the "left" and "right" side of said node.

Our goal is to start at node AAA, following the instructions until we reach node ZZZ. If we run
out of instructions we simply loop back to the start of the instruction.

What I am thinking here is we start by loading the instructions into a list of directions, making
it easy to iterate through them. Then we load the nodes into a hashmap where nodes are the keys,
and the values are a tuple with two other nodes, representing the connections.

Once the data is loaded we need a loop to iterate through the instructions. And we need a outside
loop variable to hold the current node, which should start with node AAA.
In each iteration we get the next instruction, then set the current node to the node on the side
specified by the instruction.
Then simply check if the new node is ZZZ, and stop if it is. Otherwise continue.
When stopped, return the amount of iterations performed to get the result.
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
