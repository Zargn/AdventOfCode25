#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 4361;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 557705;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 467835;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 84266818;

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

    #[derive(Default)]
    struct Schematic {
        tiles: Vec<Vec<Tile>>,
        height: usize,
        width: usize,
    }

    impl Schematic {
        fn add_row(&mut self, row: Vec<Tile>) -> Result<(), Box<dyn Error>> {
            if self.tiles.is_empty() {
                self.width = row.len();
                self.tiles.push(row);
                self.height = 1;
            } else {
                if row.len() != self.width {
                    return Err("Could not add row due to row length mismatch!".into());
                }
                self.height += 1;
                self.tiles.push(row);
            }
            Ok(())
        }

        fn scan_part_numbers(&self) -> Result<Vec<u32>, Box<dyn Error>> {
            let mut part_numbers = Vec::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Some(Tile::NumberStart(value, length)) =
                        self.tiles.get(y).and_then(|r| r.get(x))
                    {
                        if self.find_symbol(x, y, *length) {
                            part_numbers.push(*value);
                        }
                    }
                }
            }
            Ok(part_numbers)
        }

        fn find_symbol(&self, x: usize, y: usize, length: usize) -> bool {
            for f_y in 1.max(y) - 1..=y + 1 {
                for f_x in 1.max(x) - 1..=x + length + 1 {
                    if let Some(Tile::Symbol) = self.tiles.get(f_y).and_then(|r| r.get(f_x)) {
                        return true;
                    }
                }
            }
            false
        }

        fn update_number(
            tiles: &mut [Tile],
            index: usize,
            digit: u32,
        ) -> Result<(), Box<dyn Error>> {
            let Some(Tile::NumberStart(value, len)) = tiles.get_mut(index) else {
                return Err("number_builder pointed to a non-number_start tile!".into());
            };
            *value = (*value * 10) + digit;
            *len += 1;
            Ok(())
        }

        fn process_char(
            char: char,
            char_index: usize,
            number_builder: &mut Option<usize>,
            tiles: &mut Vec<Tile>,
        ) -> Result<(), Box<dyn Error>> {
            if !char.is_ascii_digit() {
                *number_builder = None;
            }
            match char {
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap(); // This will only run if c is a digit.

                    match number_builder {
                        None => {
                            tiles.push(Tile::NumberStart(digit, 0));
                            *number_builder = Some(char_index);
                        }
                        Some(start_index) => {
                            tiles.push(Tile::NumberPart);
                            Schematic::update_number(tiles, *start_index, digit)?;
                        }
                    }
                }
                '.' => tiles.push(Tile::Empty),
                _ => tiles.push(Tile::Symbol),
            }
            Ok(())
        }

        fn from_datafile(data_path: &str) -> Result<Schematic, Box<dyn Error>> {
            let mut schematic = Schematic::default();
            for line in reader::get_lines(data_path)? {
                let mut schematic_line = Vec::new();
                let mut number_builder: Option<usize> = None;
                for (char_index, char) in line.chars().enumerate() {
                    Schematic::process_char(
                        char,
                        char_index,
                        &mut number_builder,
                        &mut schematic_line,
                    )?;
                }
                schematic.add_row(schematic_line)?;
            }
            Ok(schematic)
        }
    }

    enum Tile {
        Empty,
        Symbol,
        NumberStart(u32, usize),
        NumberPart,
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let schematic = Schematic::from_datafile(data_path)?;

        let part_numbers = schematic.scan_part_numbers()?;

        Ok(part_numbers.iter().map(|u| *u as u64).sum())
    }
}

//

//

/*
Part Two
##################################################################################################

Now we need to switch it up a bit. This time we need to scan around special gear tiles instead.
We need to figure out which gears are next to exactly 2 different numbers. No more, no less.
Each time a gear has two different numbers, we calculate the "gear ratio" by multiplying the two
numbers together. Then add the result to a total.

We can use the same code as part one by tweaking it a bit.

First we need to add a Gear variant to the Tile enum.
enum Tile
    Empty,
    Symbol,
    Gear,
    NumberPart,
    NumberStart(u32, usize),

Once that is done we want to add a corresponding pattern match where we build the schematic.
'*' => schematic_line.push(Tile::Gear),

Next we need to replace the scan_part_numbers function with a scan_gears function.
The logic should be quite similar. Something like this:
For tile in surrounding 8 tiles
    if tile is numberpart(start_index)
        set tile to the numberstart tile at start_index.

    if tile is a numberstart(value, length) and this numberstart haven't been added yet
        save value and index

After checking all 8 tiles:
    Check the list of connected part_numbers.
        if the list length != 2
            return 0
        else
            return the two list values multiplied together.
*/
mod part_two {
    use crate::reader;
    use std::{collections::HashMap, error::Error};

    #[derive(Default)]
    struct Schematic {
        tiles: Vec<Vec<Tile>>,
        height: usize,
        width: usize,
    }

    impl Schematic {
        fn add_row(&mut self, row: Vec<Tile>) -> Result<(), Box<dyn Error>> {
            if self.tiles.is_empty() {
                self.width = row.len();
                self.tiles.push(row);
                self.height = 1;
            } else {
                if row.len() != self.width {
                    return Err("Could not add row due to row length mismatch!".into());
                }
                self.height += 1;
                self.tiles.push(row);
            }
            Ok(())
        }

        fn scan_gear_ratios(&self) -> Result<Vec<u64>, Box<dyn Error>> {
            let mut gear_ratios = Vec::new();
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Some(Tile::Gear) = self.tiles.get(y).and_then(|r| r.get(x)) {
                        let numbers = self.get_surrounding_part_numbers(x, y);
                        if numbers.len() != 2 {
                            continue;
                        }
                        // Unwrap is okay as this will only run IF the two values exist in the vec.
                        gear_ratios.push(
                            *numbers.first().unwrap() as u64 * *numbers.get(1).unwrap() as u64,
                        );
                    }
                }
            }
            Ok(gear_ratios)
        }

        fn get_surrounding_part_numbers(&self, x: usize, y: usize) -> Vec<u32> {
            let mut part_numbers = HashMap::new();
            for f_y in 1.max(y) - 1..=y + 1 {
                for f_x in 1.max(x) - 1..=x + 1 {
                    let Some(tile) = self.tiles.get(f_y).and_then(|r| r.get(f_x)) else {
                        continue;
                    };

                    let (part_number, start_index) = match tile {
                        Tile::NumberPart(start_index) => {
                            match self.tiles.get(f_y).and_then(|r| r.get(*start_index)) {
                                Some(Tile::NumberStart(value, _)) => (value, *start_index),
                                _ => continue,
                            }
                        }
                        Tile::NumberStart(value, _) => (value, f_x),
                        _ => continue,
                    };

                    part_numbers.insert((start_index, f_y), *part_number);
                }
            }
            part_numbers.into_values().collect()
        }

        fn update_number(
            tiles: &mut [Tile],
            index: usize,
            digit: u32,
        ) -> Result<(), Box<dyn Error>> {
            let Some(Tile::NumberStart(value, len)) = tiles.get_mut(index) else {
                return Err("number_builder pointed to a non-number_start tile!".into());
            };
            *value = (*value * 10) + digit;
            *len += 1;
            Ok(())
        }

        fn process_char(
            char: char,
            char_index: usize,
            number_builder: &mut Option<usize>,
            tiles: &mut Vec<Tile>,
        ) -> Result<(), Box<dyn Error>> {
            if !char.is_ascii_digit() {
                *number_builder = None;
            }
            match char {
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap(); // This will only run if c is a digit.

                    match number_builder {
                        None => {
                            tiles.push(Tile::NumberStart(digit, 0));
                            *number_builder = Some(char_index);
                        }
                        Some(start_index) => {
                            tiles.push(Tile::NumberPart(*start_index));
                            Schematic::update_number(tiles, *start_index, digit)?;
                        }
                    }
                }
                '.' => tiles.push(Tile::Empty),
                '*' => tiles.push(Tile::Gear),
                _ => tiles.push(Tile::Symbol),
            }
            Ok(())
        }

        fn from_datafile(data_path: &str) -> Result<Schematic, Box<dyn Error>> {
            let mut schematic = Schematic::default();
            for line in reader::get_lines(data_path)? {
                let mut schematic_line = Vec::new();
                let mut number_builder: Option<usize> = None;
                for (char_index, char) in line.chars().enumerate() {
                    Schematic::process_char(
                        char,
                        char_index,
                        &mut number_builder,
                        &mut schematic_line,
                    )?;
                }
                schematic.add_row(schematic_line)?;
            }
            Ok(schematic)
        }
    }

    enum Tile {
        Empty,
        Symbol,
        Gear,
        NumberStart(u32, usize),
        NumberPart(usize),
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let schematic = Schematic::from_datafile(data_path)?;

        let part_numbers = schematic.scan_gear_ratios()?;

        Ok(part_numbers.iter().sum())
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
