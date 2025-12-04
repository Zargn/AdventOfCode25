use std::error::Error;

mod data_parser;
mod operations;
mod reader;

/*
Part One:

This time we have a grid of either empty or occupied tiles.
We need to figure out which occupied tiles have at most 4 other occupied tiles in the surrounding 8 tiles.

We need to build a two dimensional array of tiles first.
Then iterate thorugh all tiles.
For each occupied tile we chack the surrounding 8 for other occupied tiles and count them.
If there are at most 3 other occupied tiles then add 1 to a counter.

To check the surrounding tiles for occupied tiles we could code it to only check those tiles ignoring the
current one. But it might be better to check all 9 tiles in a square and just increase the allowed tiles
by one. (Since the center tile would always be occupied.)

Once all tiles have been visited then return the count value.

Improvement:
If we give the two dimensional array 1 layer of empty tils all around the data tiles then we won't need to
check that the tile being checked is within the array.



Part Two:



*/

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Occupied,
}

impl Tile {
    fn parse(char: char) -> Result<Tile, Box<dyn Error>> {
        match char {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Occupied),
            _ => Err(format!("Tile parse error! Invalid character [{char}]!").into()),
        }
    }
}

const MAPSIZE: usize = 136 + 2; // 136 is the size of the largest data file.
                                // Add 2 for a extra layer of empty tiles around the edges.

struct Map {
    grid: [[Tile; MAPSIZE]; MAPSIZE],
    size_override: usize,
}

impl Map {
    fn load_from_file(path: &str) -> Result<Map, Box<dyn Error>> {
        let lines = reader::get_lines(path)?;
        let mut grid = [[Tile::Empty; MAPSIZE]; MAPSIZE];
        let mut size_override = 0;
        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid[x + 1][y + 1] = Tile::parse(char)?;
            }
            size_override = y + 1;
        }

        Ok(Map {
            grid,
            size_override,
        })
    }

    fn accessable_tiles(&self) -> u64 {
        let mut accessable_tiles = 0;
        for y in 1..self.size_override + 1 {
            for x in 1..self.size_override + 1 {
                if let Tile::Empty = self.grid[x][y] {
                    continue;
                }

                if self.is_accessable(x, y) {
                    accessable_tiles += 1;
                }
            }
        }
        accessable_tiles
    }

    fn is_accessable(&self, c_x: usize, c_y: usize) -> bool {
        let mut occupied = 0;
        for y in c_y - 1..=c_y + 1 {
            for x in c_x - 1..=c_x + 1 {
                if let Tile::Occupied = self.grid[x][y] {
                    occupied += 1;
                }
            }
        }
        occupied <= 4
    }

    /// Debug code to print the map in the same format as the data file.
    #[allow(dead_code)]
    fn print(&self) {
        for y in 1..self.size_override + 1 {
            for x in 1..self.size_override + 1 {
                match self.grid[x][y] {
                    Tile::Empty => print!("."),
                    Tile::Occupied => print!("@"),
                }
            }
            println!();
        }
    }
}

fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let map = Map::load_from_file(data_path)?;
    Ok(map.accessable_tiles())
}

fn main() {
    match calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_test() {
    let expected_value = 13;
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
