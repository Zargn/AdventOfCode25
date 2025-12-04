pub enum Operation {
    Integer(String),
    Float(String),
    String(String),
    None,
}

#[derive(Debug)]
pub enum OperationResult {
    Integer(i64),
    Float(f64),
    String(String),
    ParsingError(String),
    None,
}

impl Operation {
    pub fn next(self, c: char) -> (Operation, Option<OperationResult>) {
        match self {
            Operation::None => none_get_next(c),
            Operation::Integer(part) => integer_get_next(c, part),
            Operation::Float(str) => float_get_next(c, str),
            Operation::String(str) => string_get_next(c, str),
        }
    }

    // TODO: Convert ..._get_next functions to call collect_operation instead of doing the logic
    // themselves.
    pub fn collect_operation(self) -> (Operation, OperationResult) {
        match self {
            Operation::None => (Operation::None, OperationResult::None),
            Operation::Integer(str) => (
                Operation::None,
                match str.parse::<i64>() {
                    Ok(integer) => OperationResult::Integer(integer),
                    Err(_) => OperationResult::ParsingError(str),
                },
            ),
            Operation::Float(str) => (
                Operation::None,
                match str.parse::<f64>() {
                    Ok(integer) => OperationResult::Float(integer),
                    Err(_) => OperationResult::ParsingError(str),
                },
            ),
            Operation::String(str) => (Operation::None, OperationResult::String(str)),
        }
    }
}

fn none_get_next(c: char) -> (Operation, Option<OperationResult>) {
    match c {
        i if i.is_ascii_digit() || i == '-' => (Operation::Integer(String::from(c)), None),
        _ => (Operation::String(String::from(c)), None),
    }
}

fn integer_get_next(c: char, str: String) -> (Operation, Option<OperationResult>) {
    match c {
        i if i.is_ascii_digit() => {
            let mut str = str;
            str.push(c);
            (Operation::Integer(str), None)
        }
        i if i == '.' && str != "-" => {
            let mut str = str;
            str.push(c);
            (Operation::Float(str), None)
        }
        _ => {
            let result = match str.parse::<i64>() {
                Ok(integer) => OperationResult::Integer(integer),
                Err(_) => OperationResult::ParsingError(str),
            };

            let (operation, _) = none_get_next(c);
            (operation, Some(result))
        }
    }
}

fn float_get_next(c: char, str: String) -> (Operation, Option<OperationResult>) {
    match c {
        i if i.is_ascii_digit() => {
            let mut str = str;
            str.push(c);
            (Operation::Integer(str), None)
        }
        _ => {
            let result = match str.parse::<f64>() {
                Ok(float) => OperationResult::Float(float),
                Err(_) => OperationResult::ParsingError(str),
            };

            let (operation, _) = none_get_next(c);
            (operation, Some(result))
        }
    }
}

fn string_get_next(c: char, str: String) -> (Operation, Option<OperationResult>) {
    match c {
        ' ' => (Operation::None, Some(OperationResult::String(str))),
        //i if i.is_ascii_digit() || i == '-' => (Operation::Integer(String::from(c)), None),
        _ => {
            let mut str = str;
            str.push(c);
            (Operation::String(str), None)
        }
    }
}
