use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Returns a BufReader of the file at the provided path.
pub fn get_reader<P>(path: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(path)?))
}

/// Returns an iterator visiting all lines in the file at the provided path.
pub fn get_lines<P>(path: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    Ok(get_reader(path)?.lines().map_while(Result::ok))
}
