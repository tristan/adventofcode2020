use adventofcode2020::{ReadError, parse_input_file};

enum Point {
    Empty,
    Tree
}

impl Point {
    fn is_tree(&self) -> bool {
        if let Point::Tree = self { true } else { false }
    }
}

struct Map(Vec<Vec<Point>>);

impl From<Vec<Vec<Point>>> for Map {
    fn from(v: Vec<Vec<Point>>) -> Map {
        Map(v)
    }
}

impl Map {
    fn count_trees_over_slope(&self, right: usize, down: usize) -> usize {
        let x = (0..).step_by(right);
        let y = (0..).step_by(down);

        x.zip(y).take_while(|&(_, y)| y < self.0.len()).filter(|&(x, y)| {
            let row = &self.0[y];
            let point = &row[x % row.len()];
            point.is_tree()
        }).count()
    }
}

fn parse_line(line_no: usize, line: String) -> Result<Vec<Point>, ReadError> {
    let points = line.chars().map(|c| match c {
        '.' => Ok(Point::Empty),
        '#' => Ok(Point::Tree),
        _ => Err(ReadError::ParseError(line_no, format!("{}", c)))
    }).collect::<Result<Vec<Point>, ReadError>>()?;

    Ok(points)
}

fn part1(map: &Map) -> usize {
    map.count_trees_over_slope(3, 1)
}

fn part2(map: &Map) -> usize {
    map.count_trees_over_slope(1, 1) *
    map.count_trees_over_slope(3, 1) *
    map.count_trees_over_slope(5, 1) *
    map.count_trees_over_slope(7, 1) *
    map.count_trees_over_slope(1, 2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = parse_input_file("day_03_input.txt", parse_line)?;
    let result = part1(&map);
    println!("part1: {}", result);
    let result = part2(&map);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use adventofcode2020::parse_input_lines;

    #[test]
    fn test_part1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let map = parse_input_lines(input.as_bytes(), parse_line)?;

        let result = part1(&map);

        assert_eq!(result, 7);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let map = parse_input_lines(input.as_bytes(), parse_line)?;

        let result = part2(&map);

        assert_eq!(result, 336);

        Ok(())
    }
}
