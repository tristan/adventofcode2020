use std::{fs, io::{BufReader, BufRead}};
use adventofcode2020::ReadError;

struct Policy {
    min: usize,
    max: usize,
    letter: char
}

impl Policy {
    fn matches_part1(&self, password: &str) -> bool {
        let letter_counts = password
            .chars()
            .filter(|c| c == &self.letter)
            .count();
        letter_counts >= self.min && letter_counts <= self.max
    }

    fn matches_part2(&self, password: &str) -> bool {
        let mut chars = password.chars();

        let first = chars.nth(self.min - 1)
            .expect("password size doesn't fit input");
        let next = chars.nth(self.max - 1 - self.min)
            .expect("password size doesn't fit input");

        if first == self.letter {
            next != self.letter
        } else {
            next == self.letter
        }
    }
}

fn read_input_lines<R>(reader: R) -> Result<Vec<(Policy, String)>, ReadError>
where R: BufRead {
    reader.lines().enumerate().map(|(line_no, line)| {
        let line_no = line_no + 1;
        let line: String = line
            .map_err(|e| ReadError::IoError(Some(line_no), e))?;

        let (min, line) = if let Some(idx) = line.find("-") {
            let min = line[..idx].parse::<usize>()
                .map_err(|_e| ReadError::ParseError(line_no, line[..idx].to_string()))?;
            (min, &line[idx + 1..])
        } else {
            return Err(ReadError::ParseError(line_no, line));
        };

        let (max, line) = if let Some(idx) = line.find(" ") {
            let max = line[..idx].parse::<usize>()
                .map_err(|_e| ReadError::ParseError(line_no, line[..idx].to_string()))?;
            (max, &line[idx + 1..])
        } else {
            return Err(ReadError::ParseError(line_no, line.to_string()));
        };

        let (letter, line) = if let Some(idx) = line.find(":") {
            let letter = line.chars().next()
                .ok_or_else(|| ReadError::ParseError(line_no, line[..idx].to_string()))?;
            (letter, &line[idx + 2..])
        } else {
            return Err(ReadError::ParseError(line_no, line.to_string()));
        };

        let policy = Policy { min, max, letter };
        Ok((policy, line.to_string()))

    }).collect::<Result<Vec<(Policy, String)>, ReadError>>()
}

fn read_input_file(filename: &str) -> Result<Vec<(Policy, String)>, ReadError> {
    let file = fs::File::open(filename)
        .map_err(|e| ReadError::IoError(None, e))?;
    let reader = BufReader::new(file);
    read_input_lines(reader)
}

fn part1(input: &[(Policy, String)]) -> usize {
    input.iter().filter(|(policy, password)| {
        policy.matches_part1(&password)
    }).count()
}

fn part2(input: &[(Policy, String)]) -> usize {
    input.iter().filter(|(policy, password)| {
        policy.matches_part2(&password)
    }).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_file("day_02_input.txt")?;
    let result = part1(&input);
    println!("part1: {}", result);
    let result = part2(&input);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_input() -> Result<(), Box<dyn std::error::Error>> {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let data = read_input_lines(input.as_bytes())?;
        assert_eq!(data.len(), 3);
        let mut iter = data.iter();
        let item = iter.next().unwrap();
        assert_eq!(item.0.min, 1);
        assert_eq!(item.0.max, 3);
        assert_eq!(item.0.letter, 'a');
        assert_eq!(item.1, "abcde");
        let item = iter.next().unwrap();
        assert_eq!(item.0.min, 1);
        assert_eq!(item.0.max, 3);
        assert_eq!(item.0.letter, 'b');
        assert_eq!(item.1, "cdefg");
        let item = iter.next().unwrap();
        assert_eq!(item.0.min, 2);
        assert_eq!(item.0.max, 9);
        assert_eq!(item.0.letter, 'c');
        assert_eq!(item.1, "ccccccccc");

        Ok(())
    }

    #[test]
    fn test_part_1() ->  Result<(), Box<dyn std::error::Error>> {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let data = read_input_lines(input.as_bytes())?;
        let result = part1(&data);
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn test_part_2() ->  Result<(), Box<dyn std::error::Error>> {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let data = read_input_lines(input.as_bytes())?;
        let mut iter = data.iter();
        let (policy, password) = iter.next().unwrap();
        assert_eq!(policy.matches_part2(&password), true);
        let (policy, password) = iter.next().unwrap();
        assert_eq!(policy.matches_part2(&password), false);
        let (policy, password) = iter.next().unwrap();
        assert_eq!(policy.matches_part2(&password), false);
        let result = part2(&data);
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_part_2_extra() ->  Result<(), Box<dyn std::error::Error>> {
        let input = "1-3 z: aazaa";
        let data = read_input_lines(input.as_bytes())?;
        let mut iter = data.iter();
        let (policy, password) = iter.next().unwrap();
        assert_eq!(policy.matches_part2(&password), true);
        Ok(())
    }
}
