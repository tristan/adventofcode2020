use adventofcode2020::{parse_input_file, ReadError};

fn parse_bording_pass(
    line_no: usize,
    line: String
) -> Result<usize, ReadError> {
    let (row, _) = line.chars().take(7).fold(Ok((0, 0..127)), |row, c| {
        let (_, row) = row.unwrap();
        match c {
            'F' => {
                let row = row.start..(row.end - (((row.end - row.start) + 1) / 2));
                Ok((row.start, row))
            },
            'B' => {
                let row = (row.start + (((row.end - row.start) + 1) / 2))..row.end;
                Ok((row.end, row))
            },
            _ => Err(ReadError::ParseError(line_no, line.clone()))
        }
    })?;
    let (col, _) = line.chars().skip(7).fold(Ok((0, 0..7)), |col, c| {
        let (_, col) = col.unwrap();
        match c {
            'L' => {
                let col = col.start..(col.end - (((col.end - col.start) + 1) / 2));
                Ok((col.start, col))
            },
            'R' => {
                let col = (col.start + (((col.end - col.start) + 1) / 2))..col.end;
                Ok((col.end, col))
            },
            _ => Err(ReadError::ParseError(line_no, line.clone()))

        }
    })?;
    Ok(row * 8 + col)
}

fn part1(seat_ids: &[usize]) -> usize {
    *seat_ids.iter().max().unwrap()
}

fn part2(seat_ids: &mut [usize]) -> usize {
    seat_ids.sort();
    seat_ids.iter().zip(seat_ids.iter().skip(1)).find_map(|(&a, &b)| {
        if b - 1 == a { None } else { Some(b - 1) }
    }).unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seat_ids: Vec<usize> = parse_input_file(
        "day_05_input.txt", parse_bording_pass
    )?;
    let result = part1(&seat_ids);
    println!("part1: {}", result);
    let result = part2(&mut seat_ids.clone());
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_boarding_pass() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(parse_bording_pass(0, "FBFBBFFRLR".to_string())?, 44 * 8 + 5);
        assert_eq!(parse_bording_pass(0, "BFFFBBFRRR".to_string())?, 567);
        assert_eq!(parse_bording_pass(0, "FFFBBBFRRR".to_string())?, 119);
        assert_eq!(parse_bording_pass(0, "BBFFBBFRLL".to_string())?, 820);

        assert_eq!(parse_bording_pass(0, "BBBBBBBLLL".to_string())?, 127 * 8 + 0);
        assert_eq!(parse_bording_pass(0, "BBBBBBBRRR".to_string())?, 127 * 8 + 7);
        assert_eq!(parse_bording_pass(0, "FFFFFFFRRR".to_string())?, 0 * 8 + 7);
        assert_eq!(parse_bording_pass(0, "FFFFFFFLLL".to_string())?, 0 * 8 + 0);

        assert_eq!(parse_bording_pass(0, "FBFBFBFLLL".to_string())?, 42 * 8 + 0);
        assert_eq!(parse_bording_pass(0, "BFBFBFBLLL".to_string())?, 85 * 8 + 0);

        assert_eq!(parse_bording_pass(0, "FFFFFFFLRL".to_string())?, 0 * 8 + 2);
        assert_eq!(parse_bording_pass(0, "FFFFFFFRLR".to_string())?, 0 * 8 + 5);


        Ok(())
    }
}
