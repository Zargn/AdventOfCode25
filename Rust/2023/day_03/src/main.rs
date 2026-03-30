#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 4361;
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

We have a nice two dimensional array problem here.
The array contains numbers, empty spaces, and symbols. Numbers next to each other are meant to be
read together.

Our task is to figure out which numbers are "part numbers". A part number is a number if any
adjacent (including diagonal) tile is a symbol.

Then we just need to add all part numbers together to get the result.

What I am thinking here is to first create an array using the data.

(This solution is for rust, and might require quite a lot more work in other languages.)

We first need a tile enum with the following variants:
Empty,
Symbol,
NumberStart(u32, usize) // The u32 value holds the full value of this number while the usize is
                        // how many digits exist after this one in this number.
NumberPart // Since the full number and digit count is stored at the start we dont actually need
           // to make the rest of the tiles of the number anything special. We could make them
           // empty, but it feels better to give them their own variant even though it is empty.

Then we have the following 2d array: Vec<Vec<Tile>>.

When we build the array go one char at a time. If the char is:
A '.' => Set the tile to Empty,
            Set number_build to None,
A digit => If the previous tile is Empty then:
                Set this tile to NumberStart(char as u32, 0).
                Save this char index as number_build = Some(index)
           Else if number_build == Some(index)
                Set this tile to NumberPart,
                Edit the NumberStart tile at index:
                    NumberStart((u32 * 10) + this digit, usize += 1)
Anything else => Set the tile to Symbol
                 Set number_build to None


Once the array is constructed:
    For each Tile in the 2d array:
        if Tile == NumberStart(value, length)
            for y in Tile.y-1..Tile.y+1
                for x in Tile.x-1..Tile.x+NumberStart.length
                    if 2darray[x][y] = Some(Tile::Symbol)
                        add NumberStart.value to result sum.
                        break to tile loop.

What the above loop does is essentially:
iterate through all the tiles.
If a NumberStart is found then:
    Iterate through all the surrounding tiles around that NumberStart, extending
    NumberStart.length to the right.
    if any Symbol tile is found in this loop then break and add the NumberStart.value to the
    result sum.
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
