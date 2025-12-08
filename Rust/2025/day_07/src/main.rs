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

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let grid: Vec<Vec<char>> = reader::get_lines(data_path)?
        .map(|line| line.chars().collect())
        .collect();

    if grid.iter().any(|line| line.len() != grid[0].len()) {
        return Err("The data lines does not have the same lenth!".into());
    }

    let (mut beams, mut timelines): (VecDeque<(usize, usize)>, u64) = (VecDeque::new(), 0);
    for (x, c) in grid[0].iter().enumerate() {
        if *c == 'S' {
            beams.push_back((x, 0));
        }
    }

    while let Some((x, y)) = beams.pop_front() {
        if y + 1 >= grid.len() {
            timelines += 1;
            continue;
        }
        match grid[y + 1][x] {
            '.' => beams.push_back((x, y + 1)),
            '^' => {
                beams.push_back((x - 1, y + 1));
                beams.push_back((x + 1, y + 1));
            }
            _ => return Err("Invalid character in grid!".into()),
        }
    }

    Ok(timelines)
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
