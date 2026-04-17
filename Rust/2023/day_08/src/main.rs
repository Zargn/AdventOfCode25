#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 6;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 19099;

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
    use std::{collections::HashMap, error::Error};

    // The reason to return a bool array instead of a integer array is that we know the following:
    // 1: There are ONLY two types of instruction. L or R.
    // 2: There are ONLY two options to choose from in each node. Again L or R.
    // Using a bool array means we ensure the indexes provided by the instructions are always 0 or
    // 1 which always are in range of the connected nodes array attached to each node.
    // If we instead returned a integer, it would technically be possible for the index to be out
    // of range even though we would ensure it could never occur.
    fn get_instructions(data_row: &str) -> Result<Vec<bool>, Box<dyn Error>> {
        let mut instructions = Vec::new();
        for c in data_row.chars() {
            instructions.push(match c {
                'L' => false,
                'R' => true,
                _ => return Err("Invalid character found in instructions row!".into()),
            });
        }
        Ok(instructions)
    }

    fn get_nodes(
        data_lines: &mut dyn Iterator<Item = String>,
    ) -> Result<HashMap<u32, [u32; 2]>, Box<dyn Error>> {
        let mut nodes: HashMap<u32, [u32; 2]> = HashMap::new();

        for line in data_lines {
            let mut node_names = line
                .split(|c: char| !c.is_ascii_uppercase())
                .filter(|s| !s.is_empty());
            nodes.insert(
                translate_node(node_names.next().unwrap())?,
                [
                    translate_node(node_names.next().ok_or("Missing second node name!")?)?,
                    translate_node(node_names.next().ok_or("Missing third node name!")?)?,
                ],
            );
        }

        Ok(nodes)
    }

    // Translating the nodes is also technically not needed. String implements the required traits
    // for the hashmap to function using them instead. But I like converting things like this to
    // integers because it limits the type of nodes, and should in theory be more efficient as a
    // integer is primitive type compared to a full string.
    // It is also a fun challenge. :)
    fn translate_node(node_name: &str) -> Result<u32, Box<dyn Error>> {
        let (mut multiplier, mut result) = (1, 0);
        // reversing the order isn't actually needed. The program solves the puzzle just fine
        // without it. But reversing it means the leftmost char is actually the highest value one
        // instead of the lowest.
        for char in node_name.chars().rev() {
            result += (char as u8 - b'A') as u32 * multiplier;
            multiplier *= 26;
        }
        Ok(result)
    }

    // Pre-translated nodes to help readability.
    const AAA: u32 = 0;
    const ZZZ: u32 = 17575;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;

        let instructions = get_instructions(&lines.next().ok_or("Data file is empty!")?)?;
        lines.next(); // Skip empty row.

        let nodes = get_nodes(&mut lines)?;

        let mut iterations = 0;
        let mut current_node = AAA;

        while current_node != ZZZ {
            current_node = nodes
                .get(&current_node)
                .ok_or("A requested node did not exist!")?
                [instructions[iterations % instructions.len()] as usize];
            iterations += 1;
        }

        Ok(iterations as u64)
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
