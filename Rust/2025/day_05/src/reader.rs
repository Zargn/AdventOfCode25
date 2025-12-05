use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_reader<P>(path: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

pub fn get_lines<P>(path: P) -> io::Result<std::iter::Flatten<io::Lines<io::BufReader<File>>>>
where
    P: AsRef<Path>,
{
    let bufreader = get_reader(path)?;

    Ok(bufreader.lines().flatten())
}
