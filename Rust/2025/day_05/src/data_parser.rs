//TODO

use crate::operations;

pub struct ParserRule {}

pub struct Pattern {}

pub enum ParseResult {
    String(String),
    Char(char),
    Float(f64),
    Integer(i64),
}

pub struct DataParser {
    data_iter: Box<dyn Iterator<Item = String>>,
    rules: Vec<ParserRule>,
}

impl DataParser {
    pub fn new(rules: Vec<ParserRule>, data_iter: Box<dyn Iterator<Item = String>>) -> Self {
        todo!();
    }

    pub fn read_next() -> Option<ParseResult> {
        todo!();
    }
}

trait ParsePattern {
    fn pattern() {}
}

// We want to have some kind of pattern struct so the user can pass in structs to be produced by
// the parser.
// The structs should contain the expected format and the logic to build from the matching data.
// Each part to be loaded should be separate and be of one type.
// So a part also has a expected format. Which could include parts that wont be in the collected
// value. For example a float might have the pattern: "x:(0-9).(0-9)" which would produce a float,
// So x:4.2 would result in the float 4.2
