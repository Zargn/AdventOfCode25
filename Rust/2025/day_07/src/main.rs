use std::{collections::VecDeque, error::Error};

mod data_parser;
mod operations;
mod reader;

/*
Part One:

We have a grid of empty spaces '.' and splitters '^'. The goal is to figure out how many
times a splitter is hit.
The beam starts at position 'S' and always travels downwards. If a splitter is hit the
beam stops and two new beams start on both sides of the splitter.

Splitters might split a beam into another beam. In those cases it still only counts as
one beam.

The "obvious" solution here would be to simulate the beam going downwards, and while the
goal could be reached by other methods as well, any other I can think of would be far
more complex.

So to simulate the beam:

Load the data into a grid
Set the tile at location 'S' to a beam

Iterate through the grid line by line until the second last line
    Iterate through each tile of the line
        If the tile is a beam:
            If the tile below the current is a:
            Empty space/Beam:
                Set tile below to a beam
            Splitter:
                Set the tiles next to the one below to beams
                Increase split counter by one
return split count

No need to check that the split beam is inside the grid as the closest a splitter is to
the edge is at least one empty tile.



Part Two:

New we need to figure out how many different paths one beam can take. At first I was
thinking this might be possible to solve with math alone, but I am too tired to figure
that out at this moment...

So, counting paths is the answer instead. Since one tile might carry multiple paths
we can't use the same loop as before.

I am thinking we use a queue that holds a "beam" struct, which would essentially be a
2D vector representing the location of the beam.

Loop dequeuing one beam at a time until the queue is empty
    If the tile below the current is a:
        Empty space:
            Increase beam y position by one and add back to queue
        Splitter:
            Create two new beams with position y+1 x-1 and y+1 x+1
            Add beams back to queue
    Else If the beam has reached the bottom of the grid:
        Increase timeline count by 1.

Return timeline count

Update:
This solution works for the test value but takes ages for the full data.
I think it will get to the correct solution but I don't know if it takes 10 minutes or 5
years. Need to redesign it to be more efficient.

We need to stop processing the same split multiple times.
Ideally we would process the grid one line at a time again. But instead of just ignoring
if a beam exists at a location we save the beam with a value. Where the value holds how
many beams have reached that tile. Then when a split occurs both split beams get that
same value, plus any potential existing value from other beams.

If we modify the original solution:

Load the data into a grid
Set the tile at location 'S' to a beam

Iterate through the grid line by line until the second last line
    Iterate through each tile of the line
        If the tile is a beam:
            If the tile below the current is a:
            Empty space/Beam:
                Set tile below to a beam
            Splitter:
                Set the tiles next to the one below to beams
                Increase split counter by one
return split count

to this:

Load the data into a grid
Set the tile at location 'S' to a beam

beam_strenght hashmap with (x, y) as a key and a int as a value.

Iterate through the grid line by line until the second last line
    Iterate through each tile of the line
        If the tile is a beam:
            If the tile below the current is a:
            Empty space:
                Set tile below to a beam of the same strenght as the current
            Beam:
                Add the current beam strenght to the existing beam strenght
            Splitter:
                Add beams with the same strenght as the current to the two tiles
                  next to the splitter

iterate through the last line of the grid adding the beam strenghts together
and return the result



Rust allows the use of Enums with values, so I use that instead of the hashmap
approach, but the logic behind it is the same. Just instead of storing the
strength in the hashmap I store it in the Beam type of tiles.



*/

fn calculate_part_one(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut grid: Vec<Vec<char>> = reader::get_lines(data_path)?
        .map(|line| line.chars().collect())
        .collect();

    let (grid_width, mut splits) = (grid[0].len(), 0);
    if grid.iter().any(|line| line.len() != grid_width) {
        return Err("The data lines does not have the same lenth!".into());
    }

    for y in 0..grid.len() - 1 {
        for x in 0..grid_width {
            if grid[y][x] == '|' || grid[y][x] == 'S' {
                match grid[y + 1][x] {
                    '.' | '|' => grid[y + 1][x] = '|',
                    '^' => {
                        grid[y + 1][x - 1] = '|';
                        grid[y + 1][x + 1] = '|';
                        splits += 1;
                    }
                    _ => return Err("Invalid character in grid!".into()),
                }
            }
        }
    }

    Ok(splits)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Beam(u64),
    Empty,
    Splitter,
}

impl Tile {
    /// If the tile this is called on is of type Beam or Empty a Beam type will be returned with
    /// the strength provided + any potential old strength of an existing beam.
    /// If the tile is a Splitter then a splitter is returned.
    fn add_beam_strenght(&self, strength: u64) -> Tile {
        match self {
            Tile::Empty => Tile::Beam(strength),
            Tile::Beam(current_strength) => Tile::Beam(current_strength + strength),
            Tile::Splitter => Tile::Splitter,
        }
    }

    fn parse(c: &char) -> Result<Tile, Box<dyn Error>> {
        match c {
            'S' => Ok(Tile::Beam(1)),
            '.' => Ok(Tile::Empty),
            '^' => Ok(Tile::Splitter),
            _ => Err("Invalid character in data file!".into()),
        }
    }
}

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    for line in reader::get_lines(data_path)? {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::parse(&c)?);
        }
        grid.push(row);
    }

    if grid.iter().any(|line| line.len() != grid[0].len()) {
        return Err("The data lines does not have the same lenth!".into());
    }

    for y in 0..grid.len() - 1 {
        for x in 0..grid[0].len() {
            if let Tile::Beam(strength) = grid[y][x] {
                match grid[y + 1][x] {
                    Tile::Beam(_) | Tile::Empty => {
                        grid[y + 1][x] = grid[y + 1][x].add_beam_strenght(strength)
                    }
                    Tile::Splitter => {
                        grid[y + 1][x - 1] = grid[y + 1][x - 1].add_beam_strenght(strength);
                        grid[y + 1][x + 1] = grid[y + 1][x + 1].add_beam_strenght(strength);
                    }
                }
            }
        }
    }

    let mut c = 0;
    for tile in grid[grid.len() - 1].iter() {
        if let Tile::Beam(strength) = tile {
            c += strength;
        }
    }
    Ok(c)
}

fn main() {
    match calculate_part_one("data.txt") {
        Ok(value) => println!("Part One Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
    match calculate_part_two("data.txt") {
        Ok(value) => println!("Part Two Result:\n{}", value),
        Err(err) => println!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_part_one_test() {
    let expected_value = 21;
    match calculate_part_one("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Program using testdata.txt finished but result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Error occured:\n{}", err),
    }
}

#[test]
fn calculate_part_two_test() {
    let expected_value = 40;
    match calculate_part_two("testdata.txt") {
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
