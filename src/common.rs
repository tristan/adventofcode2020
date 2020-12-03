use std::{fs, fmt, io::{self, BufReader, BufRead}, str::FromStr};

#[derive(Debug)]
pub enum ReadError {
    IoError(Option<usize>, io::Error),
    ParseError(usize, String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReadError {}

pub fn read_input_lines<T: FromStr>(filename: &str) -> Result<Vec<T>, ReadError> {
    let file = fs::File::open(filename)
        .map_err(|e| ReadError::IoError(None, e))?;
    let reader = BufReader::new(file);
    let result: Result<Vec<T>, ReadError> = reader.lines().enumerate().map(|(line_no, line)| {
        let line_no = line_no + 1;
        let line: String = line
            .map_err(|e| ReadError::IoError(Some(line_no), e))?;
        Ok(line.parse::<T>().map_err(|_e| ReadError::ParseError(line_no, line))?)
    }).collect();
    result
}

pub fn parse_input_lines<V, R, T, F>(reader: R, f: F) -> Result<V, ReadError>
where F: Fn(usize, String) -> Result<T, ReadError>,
      V: From<Vec<T>>,
      R: BufRead
{
    let rows = reader.lines().enumerate().map(|(line_no, line)| {
        let line_no = line_no + 1;
        let line: String = line
            .map_err(|e| ReadError::IoError(Some(line_no), e))?;

        Ok(f(line_no, line)?)
    }).collect::<Result<Vec<T>, ReadError>>()?;

    Ok(rows.into())
}

pub fn parse_input_file<V, T, F>(filename: &str, f: F) -> Result<V, ReadError>
where F: Fn(usize, String) -> Result<T, ReadError>,
      V: From<Vec<T>>
{
    let file = fs::File::open(filename)
        .map_err(|e| ReadError::IoError(None, e))?;
    let reader = BufReader::new(file);
    parse_input_lines(reader, f)
}
