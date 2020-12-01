use std::{fs, fmt, io::{self, BufReader, BufRead}, str::FromStr};

#[derive(Debug)]
pub enum ReadError {
    IoError(io::Error),
    ParseError
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReadError {}

pub fn read_input_lines<T: FromStr>(filename: &str) -> Result<Vec<T>, ReadError> {
    let file = fs::File::open(filename)
        .map_err(|e| ReadError::IoError(e))?;
    let reader = BufReader::new(file);
    let result: Result<Vec<T>, ReadError> = reader.lines().map(|line| {
        let line: String = line
            .map_err(|e| ReadError::IoError(e))?;
        Ok(line.parse::<T>().map_err(|_e| ReadError::ParseError)?)
    }).collect();
    result
}
