use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Vec<Direction>> {
    reader.lines().filter_map(Result::ok)
        .map(|line| {
            let mut chars = line.chars();
            let mut directions = vec![];
            while let Some(c) = chars.next() {
                match c {
                    'e' => directions.push(Direction::East),
                    's' => {
                        match chars.next() {
                            Some('w') => directions.push(Direction::SouthWest),
                            Some('e') => directions.push(Direction::SouthEast),
                            _ => panic!("invalid input")
                        }
                    },
                    'w' => directions.push(Direction::West),
                    'n' => {
                        match chars.next() {
                            Some('w') => directions.push(Direction::NorthWest),
                            Some('e') => directions.push(Direction::NorthEast),
                            _ => panic!("invalid input")
                        }
                    },
                    _ => panic!("invalid input")
                }
            }
            directions
        }).collect()
}

enum Tile {
    Black,
    White
}

struct Floor(HashMap<(isize, isize), Tile>);

impl Floor {
    fn new() -> Floor {
        Floor(HashMap::new())
    }

    fn identify(&self, x: isize, y: isize, d: Direction) -> (isize, isize) {
        let (nx, ny) = match d {
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
            Direction::SouthEast => if y % 2 == 0 {
                (x + 1, y + 1)
            } else {
                (x, y + 1)
            },
            Direction::SouthWest => if y % 2 == 0 {
                (x, y + 1)
            } else {
                (x - 1, y + 1)
            },
            Direction::NorthEast => if y % 2 == 0 {
                (x + 1, y - 1)
            } else {
                (x, y - 1)
            },
            Direction::NorthWest => if y % 2 == 0 {
                (x, y - 1)
            } else {
                (x - 1, y - 1)
            },
        };

        (nx, ny)
    }

    fn flip(&mut self, x: isize, y: isize) {
        let new_tile = if let Some(tile) = self.0.get(&(x, y)) {
            match tile {
                Tile::White => Tile::Black,
                Tile::Black => Tile::White
            }
        } else {
            Tile::Black
        };
        self.0.insert((x, y), new_tile);
    }

    fn count_black_tiles(&self) -> usize {
        self.0.values().filter(|t| match t { Tile::Black => true, _ => false })
            .count()
    }

    fn step(&self) -> Floor {
        let mut new_tiles = HashMap::new();
        let mut neighbors = HashMap::new();
        self.0.iter()
            .filter(|(_, tile)| match tile { Tile::Black => true, _ => false })
            .for_each(|(&(x, y), _)| {
                [
                    Direction::West, Direction::NorthWest, Direction::SouthWest,
                    Direction::East, Direction::NorthEast, Direction::SouthEast
                ].iter().map(|&d| {
                    let pos = self.identify(x, y, d);
                    let c: usize = neighbors.get(&pos).copied()
                        .unwrap_or(0) + 1;
                    neighbors.insert(pos, c);
                }).count();
                if neighbors.get(&(x, y)).is_none() {
                    neighbors.insert((x, y), 0);
                }
            });
        neighbors.iter()
            .for_each(|(&(x, y), &count)| {
                match self.0.get(&(x, y)) {
                    Some(Tile::White) | None => {
                        if count == 2 {
                            new_tiles.insert((x, y), Tile::Black);
                        }
                    },
                    Some(Tile::Black) => {
                        if !(count == 0 || count > 2) {
                            new_tiles.insert((x, y), Tile::Black);
                        }
                    }
                }
            });
        Floor(new_tiles)
    }
}

fn initialize_floor(directions: &Vec<Vec<Direction>>) -> Floor {
    let mut floor = Floor::new();
    directions.iter().for_each(|row| {
        let (x, y) = row.iter().fold((0, 0), |(x, y), &d| {
            floor.identify(x, y, d)
        });
        floor.flip(x, y);
    });
    floor
}

fn part1(floor: &Floor) -> usize {
    floor.count_black_tiles()
}

fn part2(floor: Floor) -> usize {
    let floor = (0..100).fold(floor, |floor, _| {
        floor.step()
    });
    floor.count_black_tiles()
}

fn main() {
    let f = File::open("day_24_input.txt").unwrap();
    let directions = parse_input(BufReader::new(f));
    let floor = initialize_floor(&directions);
    let result = part1(&floor);
    println!("part1: {}", result);
    let result = part2(floor);
    println!("part2: {}", result);

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

        let directions = parse_input(input.as_bytes());
        let floor = initialize_floor(&directions);
        let result = part1(&floor);
        assert_eq!(result, 10);
        let result = part2(floor);
        assert_eq!(result, 2208);
    }
}
