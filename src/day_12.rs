use std::{fs, io::{BufReader, BufRead}};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

const RIGHT_DIRS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];
const LEFT_DIRS: [Direction; 4] = [Direction::West, Direction::South, Direction::East, Direction::North];

impl Direction {
    fn right(&self, arg: i32) -> Direction {
        let arg = (arg % 360 / 90) as usize;
        let s: usize = match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        RIGHT_DIRS[(s + arg) % 4]
    }
    fn left(&self, arg: i32) -> Direction {
        let arg = (arg % 360 / 90) as usize;
        let s: usize = match self {
            Direction::North => 3,
            Direction::East => 2,
            Direction::South => 1,
            Direction::West => 0,
        };
        LEFT_DIRS[(s + arg) % 4]
    }
}

fn part1<R>(reader: R) -> i32
where R: BufRead {
    let (_, x, y) = reader.lines().enumerate()
        .fold((Direction::East, 0i32, 0i32), |state, (line_no, line)| {
            let line = line
                .unwrap_or_else(|_| panic!("Error reading line: {}", line_no + 1));
            let (command, argument) = line.split_at(1);
            let argument = argument.parse::<i32>()
                .unwrap_or_else(|_| panic!("Error parsing argument on line: {}", line_no + 1));
            match command {
                "N" => (state.0, state.1, state.2 + argument),
                "S" => (state.0, state.1, state.2 - argument),
                "E" => (state.0, state.1 + argument, state.2),
                "W" => (state.0, state.1 - argument, state.2),
                "L" => (state.0.left(argument), state.1, state.2),
                "R" => (state.0.right(argument), state.1, state.2),
                "F" => match state.0 {
                    Direction::North => (state.0, state.1, state.2 + argument),
                    Direction::South => (state.0, state.1, state.2 - argument),
                    Direction::East => (state.0, state.1 + argument, state.2),
                    Direction::West => (state.0, state.1 - argument, state.2),
                },
                _ => panic!("Invalid command on line: {}", line_no + 1)
            }
        });
    x.abs() + y.abs()
}

fn part2<R>(reader: R) -> i32
where R: BufRead {
    let (_, _, sx, sy) = reader.lines().enumerate()
        .fold((10i32, 1i32, 0i32, 0i32), |(wx, wy, sx, sy), (line_no, line)| {
            let line = line
                .unwrap_or_else(|_| panic!("Error reading line: {}", line_no + 1));
            let (command, argument) = line.split_at(1);
            let argument = argument.parse::<i32>()
                .unwrap_or_else(|_| panic!("Error parsing argument on line: {}", line_no + 1));
            match command {
                "N" => (wx, wy + argument, sx, sy),
                "S" => (wx, wy - argument, sx, sy),
                "E" => (wx + argument, wy, sx, sy),
                "W" => (wx - argument, wy, sx, sy),
                "L" => {
                    let (wx, wy) = (0..(argument % 360 / 90)).fold((wx, wy), |(x, y), _| {
                        (-y, x)
                    });
                    (wx, wy, sx, sy)
                },
                "R" => {
                    let (wx, wy) = (0..(argument % 360 / 90)).fold((wx, wy), |(x, y), _| {
                        (y, -x)
                    });
                    (wx, wy, sx, sy)
                },
                "F" => {
                    (wx, wy, sx + wx * argument, sy + wy * argument)
                },
                _ => panic!("Invalid command on line: {}", line_no + 1)
            }
        });
    sx.abs() + sy.abs()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("day_12_input.txt")?;
    let reader = BufReader::new(file);
    let result = part1(reader);
    println!("part1: {}", result);

    let file = fs::File::open("day_12_input.txt")?;
    let reader = BufReader::new(file);
    let result = part2(reader);
    println!("part2: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"F10
N3
F7
R90
F11
"#;
        assert_eq!(part1(input.as_bytes()), 25);
        assert_eq!(part2(input.as_bytes()), 286);
        Ok(())
    }
}
