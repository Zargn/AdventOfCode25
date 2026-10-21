#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 46;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 6978;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 51;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

This is a rather unique challenge, as I actually already completed part one in a previous attempt.
Which means we have access to both the expected value for part one, and the information for the
second puzzle.

We should try and make a solution for part one that will be as compatible with part two as
possible.

And in fact, part one and two both rely on the same logic for "simulating" the beams. The only
difference in part two is that we don't have a set start edge and instead need to check all edges
to find which one results in the most energized tiles. So our goal is to make a "simulate beam"
function.

We can also make this function take a reference to the map instead of cloning it. That way we
don't need to clone it for each simulation in part two.

We want to start by creating a grid where each cell can be 3 different variants.
- Empty
- Mirror
- Splitter

We will also likely want a "Direction" enum to make it easier to change and understand beam
directions.


Each variant also needs a energized_by: u16 value. The reason to have this as a u16 instead of a
bool is to avoid having to clear the beam simulation info from the map. Instead of comparing
true/false we check if the value matches a "beam_id" we received from the caller. This way we
avoid having to clone the original non-modified grid OR manually iterating through the grid
reseting all the values each time we simulate a new beam.
This should also be fairly easy to benchmark later.
*/
mod part_one {
    use crate::reader;
    use std::{error::Error, mem};

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    struct Vector {
        x: i16,
        y: i16,
    }

    #[derive(Clone, Copy)]
    enum Direction {
        North = 0,
        East = 1,
        South = 2,
        West = 3,
    }

    impl Direction {
        fn as_vector(&self) -> Vector {
            match self {
                Self::North => Vector { x: 0, y: -1 },
                Self::East => Vector { x: 1, y: 0 },
                Self::South => Vector { x: 0, y: 1 },
                Self::West => Vector { x: -1, y: 0 },
            }
        }
    }

    struct Beam {
        position: Vector,
        direction: Direction,
        id: u16,
    }

    impl Beam {
        fn next(&self, direction: Direction) -> Beam {
            let dir = direction.as_vector();
            Beam {
                position: Vector {
                    x: self.position.x + dir.x,
                    y: self.position.y + dir.y,
                },
                direction,
                id: self.id,
            }
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    enum CellType {
        #[default]
        Empty,
        LeftMirror,
        RightMirror,
        VerticalSplitter,
        HorizontalSplitter,
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct Cell {
        cell_type: CellType,
        energized_by: u16,
        last_visits: [u16; 4],
    }

    impl Cell {
        fn of_type(cell_type: CellType) -> Cell {
            Cell {
                cell_type,
                energized_by: 0,
                last_visits: [0; 4],
            }
        }

        fn simulate_beam(&mut self, beam: Beam, beam_cache: &mut Vec<Beam>) -> bool {
            let energized = mem::replace(&mut self.energized_by, beam.id) != beam.id;

            if mem::replace(&mut self.last_visits[beam.direction as usize], beam.id) == beam.id {
                return energized;
            }

            let new_dir = match self.cell_type {
                CellType::Empty => beam.direction,
                CellType::LeftMirror => match beam.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                },
                CellType::RightMirror => match beam.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                },
                CellType::VerticalSplitter => match beam.direction {
                    Direction::West | Direction::East => {
                        beam_cache.push(beam.next(Direction::North));
                        Direction::South
                    }
                    _ => beam.direction,
                },
                CellType::HorizontalSplitter => match beam.direction {
                    Direction::North | Direction::South => {
                        beam_cache.push(beam.next(Direction::East));

                        Direction::West
                    }
                    _ => beam.direction,
                },
            };
            beam_cache.push(beam.next(new_dir));

            energized
        }

        fn from_char(c: char) -> Result<Cell> {
            Ok(match c {
                '.' => Cell::of_type(CellType::Empty),
                '\\' => Cell::of_type(CellType::LeftMirror),
                '/' => Cell::of_type(CellType::RightMirror),
                '|' => Cell::of_type(CellType::VerticalSplitter),
                '-' => Cell::of_type(CellType::HorizontalSplitter),
                _ => return Err(format!("Unsupported char in data file! [{c}]").into()),
            })
        }
    }

    struct Map {
        grid: [[Cell; 110]; 110],
        size_override: usize,
    }

    impl Map {
        fn in_range(&self, position: &Vector) -> bool {
            (0..=self.size_override as i16).contains(&position.x)
                && (0..=self.size_override as i16).contains(&position.y)
        }

        fn simulate_beam(&mut self, beam: Beam) -> u64 {
            let mut cells_energized = 0;
            let mut beams = vec![beam];
            while !beams.is_empty() {
                let beam = beams.remove(0);
                if self.in_range(&beam.position) {
                    let newly_energized = self.grid[beam.position.y as usize]
                        [beam.position.x as usize]
                        .simulate_beam(beam, &mut beams);
                    cells_energized += newly_energized as u64;
                }
            }
            cells_energized
        }

        fn from_data(data_lines: impl Iterator<Item = String>) -> Result<Map> {
            let mut grid = [[Cell::default(); 110]; 110];
            let mut size_override = 0;
            for (y, line) in data_lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    grid[y][x] = Cell::from_char(c)?;
                }
                size_override = y;
            }
            Ok(Map {
                grid,
                size_override,
            })
        }

        fn draw_map(&self, beam_id: u16) {
            for y in 0..=self.size_override {
                for x in 0..=self.size_override {
                    if self.grid[y][x].energized_by == beam_id {
                        print!("#");
                    } else {
                        print!(" ")
                    }
                }
                println!();
            }
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64> {
        let mut map = Map::from_data(reader::get_lines(data_path)?)?;
        let energized_tiles = map.simulate_beam(Beam {
            id: 1,
            position: Vector { x: 0, y: 0 },
            direction: Direction::East,
        });

        //map.draw_map(1);

        Ok(energized_tiles)
    }
}

//

//

/*
Part Two
##################################################################################################

This time we are not meant to just simulate the beam from the top left, but rather we need to
simulate one beam for each position along each edge. Then we want to figure out which beam
energized the most tiles. The amount found is the answer we seek.

Thanks to me knowing the Part Two puzzle beforehand I was able to build the part one solution to
only require some slight modifications to solve this part.

The beam simulation code doesn't need any changing at all. We just need to simulate more
beams and save the one that energized the most tiles.

After copying the Part One code all we need to do is iterate through each cell along the edges.
(Note that cells in the 4 corners needs to have two beams simulated from them as they connect to
two edges.)

One way to do this would be to:
Have two separate loops handling vertical and horizontal edges.

mut beam_id = 1;

(Horizontal edge)
for x in 0..map.size_override
    Top edge:
    map.simulate_beam(Beam {
        id: beam_id,
        position: Vector { x: x, y: 0 },
        direction: Direction::South,
    });
    beam_id += 1;

    Bottom edge:
    map.simulate_beam(Beam {
        id: beam_id,
        position: Vector { x: x, y: map.size_override },
        direction: Direction::North,
    });
    beam_id += 1;

Then basically the same again but for the y axis instead.
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn calculate(data_path: &str) -> Result<u64> {
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
