#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 405;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 29846;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 400;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 25401;

//

//

/*
Part One
##################################################################################################

2D array time.
We have a number of maps that contain ash '.' and rocks '#'. Our goal is to find a mirror line.
Basically, each map has either a horizontal or vertical mirror line where the two sides mirror
each other."#..#..#.." would have a mirror here: ".#..#.|.#...##". We see the mirror line, but not
all indexes are mirrored. So what we need to do is not only to find the mirror line, but also how
many rows/columns on each side actually mirror each other.

The approach I am thinking of here is actually to only use the 2d array when loading the maps.
We need to compare complete rows and columns, and since they can only contain two different
values, we should be able to convert each line and column into a binary number. Then we can just
compare the numbers instead of the rows, since technically the numbers are the columns and rows.

But there is one issue that needs to be explored. The puzzle instructions says to "find a perfect
reflection across EITHER a horizontal or vertical line". Making it sound like each map should only
contain 1 possible mirror line that is either horizontal or vertical.
This however doesn't match the example data OR the full data. Both of them contain maps that has
both horizontal and vertical mirror lines.

The instructions does not say how to handle these.
One option is to combine them if both a horizontal and vertical is found, but then that would make
the second example map result in a incorrect value.
Update: I think I figured it out. They say we need to "find a perfect reflection across either a
horizontal line between two rows or...". My issue with this is that makes it sound like what we
need to find are the mirror line and then count rows/columns to get the result. But what they
appear to mean is that for a mirror line to be valid it not only needs two identical rows/columns
next to each other, but also ALL surrounding rows/columns needs to have a mirror version at the
same offset from the found mirror line. Any row/column whose mirror would be outside of the map
can be ignored.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    fn to_int(bits: [u8; 17]) -> u32 {
        let mut result = 0;
        for (i, bit) in bits.iter().enumerate() {
            result += (*bit as u32) << i;
        }
        result
    }

    fn remove_empty(list: &mut Vec<u32>) {
        while let Some(last_value) = list.last() {
            if *last_value == 0 {
                list.remove(list.len() - 1);
            } else {
                break;
            }
        }
    }

    fn into_values(grid: [[u8; 17]; 17]) -> (Vec<u32>, Vec<u32>) {
        let mut rows = Vec::new();
        for row in grid {
            rows.push(to_int(row));
        }
        remove_empty(&mut rows);

        let mut columns = Vec::new();
        for x in 0..17 {
            let mut column = [0; 17];
            for (y, row) in grid.iter().enumerate() {
                column[y] = row[x];
            }
            columns.push(to_int(column));
        }
        remove_empty(&mut columns);

        (rows, columns)
    }

    fn is_mirrored(values: &[u32]) -> bool {
        if values[0] == values[values.len() - 1] {
            if values.len() == 2 {
                return true;
            }
            return is_mirrored(&values[1..values.len() - 1]);
        }
        false
    }

    fn try_get_mirror_line_index(values: Vec<u32>) -> Option<u64> {
        let len = values.len();

        for i in (1..len).rev() {
            if values[0] == values[i] && i % 2 == 1 && is_mirrored(&values[0..i + 1]) {
                return Some((i + 1) as u64 / 2);
            }
        }

        for i in 0..len - 1 {
            if values[len - 1] == values[i] && (len - i) % 2 == 0 && is_mirrored(&values[i..len]) {
                return Some((len - ((len - i) / 2)) as u64);
            }
        }

        None
    }

    fn calculate_mirror_line_score(
        rows: Vec<u32>,
        columns: Vec<u32>,
    ) -> Result<u64, Box<dyn Error>> {
        Ok(
            match (
                try_get_mirror_line_index(rows),
                try_get_mirror_line_index(columns),
            ) {
                (Some(value), None) => value * 100,
                (None, Some(value)) => value,
                (Some(value), Some(value2)) => {
                    return Err(
                        format!("Found two lines! row {} and column {}", value, value2).into(),
                    )
                }
                _ => return Err("Couldn't find any mirror lines!".into()),
            },
        )
    }

    fn process_next(
        data_lines: &mut impl Iterator<Item = String>,
    ) -> Result<Option<u64>, Box<dyn Error>> {
        let mut grid = [[0; 17]; 17];

        let mut empty = true;

        for (y, line) in data_lines.enumerate() {
            empty = false;
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = match c {
                    '#' => 1,
                    '.' => 0,
                    _ => return Err(format!("Found unexpected char [{c}] in data file!").into()),
                }
            }
        }

        if empty {
            return Ok(None);
        }

        let (rows, columns) = into_values(grid);
        Ok(Some(calculate_mirror_line_score(rows, columns)?))
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;
        let mut result = 0;

        while let Some(score) = process_next(&mut lines)? {
            result += score;
        }

        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

So now we are supposed to first "clean" the mirrors.
Basically, each mirror has one character that is flipped. If that character is flipped then a new
mirror line will be possible.
When we find this new mirror line we calculate the score the same way as before.

What I am thinking is that we can use the same code as part one, but tweak the functions that
searches for mirror lines a little.
Since we know the mirror line we are looking for has ONE character that has been modified, the
solution might actually be quite simple.
If we add a bool "flipped" to the is_mirrored function and using that allow one flip for each
is_mirrored chain then we should be able to easily get the new answer.
However this assumes a few things that I am usure of.
1: It assumes that there is only ONE possible mirror line per map where a character was flipped.
2: It assumes that there is no rows/columns where more than one character is flipped.

This can also be fixed by only allowing "flipped" if the tvo values actually only differ by one
bit.
Edit: assumption 2 was wrong. So I added a method to only allow a "flip" if the numbers differ by
one bit.
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    fn to_int(bits: [u8; 17]) -> u32 {
        let mut result = 0;
        for (i, bit) in bits.iter().enumerate() {
            result += (*bit as u32) << i;
        }
        result
    }

    fn remove_empty(list: &mut Vec<u32>) {
        while let Some(last_value) = list.last() {
            if *last_value == 0 {
                list.remove(list.len() - 1);
            } else {
                break;
            }
        }
    }

    fn into_values(grid: [[u8; 17]; 17]) -> (Vec<u32>, Vec<u32>) {
        let mut rows = Vec::new();
        for row in grid {
            rows.push(to_int(row));
        }
        remove_empty(&mut rows);

        let mut columns = Vec::new();
        for x in 0..17 {
            let mut column = [0; 17];
            for (y, row) in grid.iter().enumerate() {
                column[y] = row[x];
            }
            columns.push(to_int(column));
        }
        remove_empty(&mut columns);

        (rows, columns)
    }

    fn differs_by_1_bit(value: u32, other: u32) -> bool {
        (value ^ other).is_power_of_two()
    }

    fn is_mirrored(values: &[u32], flipped: bool) -> bool {
        if values[0] == values[values.len() - 1] {
            if values.len() == 2 {
                return flipped;
            }
            return is_mirrored(&values[1..values.len() - 1], flipped);
        }
        if !flipped && differs_by_1_bit(values[0], values[values.len() - 1]) {
            if values.len() == 2 {
                return true;
            }

            return is_mirrored(&values[1..values.len() - 1], true);
        }
        false
    }

    fn try_get_mirror_line_index(values: Vec<u32>) -> Option<u64> {
        let len = values.len();

        for i in (1..len).rev() {
            if i % 2 == 1 {
                if values[0] == values[i] {
                    if is_mirrored(&values[0..i + 1], false) {
                        return Some((i + 1) as u64 / 2);
                    }
                } else if is_mirrored(&values[0..i + 1], false) {
                    return Some((i + 1) as u64 / 2);
                }
            }
        }
        for i in 0..len - 1 {
            if (len - i) % 2 == 0 {
                if values[len - 1] == values[i] {
                    if is_mirrored(&values[i..len], false) {
                        return Some((len - ((len - i) / 2)) as u64);
                    }
                } else if is_mirrored(&values[i..len], false) {
                    return Some((len - ((len - i) / 2)) as u64);
                }
            }
        }

        None
    }

    fn calculate_mirror_line_score(
        rows: Vec<u32>,
        columns: Vec<u32>,
    ) -> Result<u64, Box<dyn Error>> {
        Ok(
            match (
                try_get_mirror_line_index(rows),
                try_get_mirror_line_index(columns),
            ) {
                (Some(value), None) => value * 100,
                (None, Some(value)) => value,
                (Some(value), Some(value2)) => {
                    return Err(
                        format!("Found two lines! row {} and column {}", value, value2).into(),
                    )
                }
                _ => return Err("Couldn't find any mirror lines!".into()),
            },
        )
    }

    fn process_next(
        data_lines: &mut impl Iterator<Item = String>,
    ) -> Result<Option<u64>, Box<dyn Error>> {
        let mut grid = [[0; 17]; 17];

        let mut empty = true;

        for (y, line) in data_lines.enumerate() {
            empty = false;
            if line.is_empty() {
                break;
            }
            for (x, c) in line.chars().enumerate() {
                grid[y][x] = match c {
                    '#' => 1,
                    '.' => 0,
                    _ => return Err(format!("Found unexpected char [{c}] in data file!").into()),
                }
            }
        }

        if empty {
            return Ok(None);
        }

        let (rows, columns) = into_values(grid);
        Ok(Some(calculate_mirror_line_score(rows, columns)?))
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut lines = reader::get_lines(data_path)?;
        let mut result = 0;

        while let Some(score) = process_next(&mut lines)? {
            result += score;
        }

        Ok(result)
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
