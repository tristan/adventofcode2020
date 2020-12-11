use adventofcode2020::{ReadError, parse_input_file};
use std::iter::repeat;

#[derive(Clone, Copy)]
enum Tile {
    EmptySeat,
    Occupied,
    Floor
}

impl Tile {
    fn is_occupied(&self) -> bool {
        match self {
            Tile::Occupied => true,
            _ => false
        }
    }

    fn is_floor(&self) -> bool {
        match self {
            Tile::Floor => true,
            _ => false
        }
    }
}

impl Into<usize> for Tile {
    fn into(self) -> usize {
        match self {
            Tile::Occupied => 1,
            _ => 0
        }
    }
}

#[derive(Clone)]
struct State(Vec<Vec<Tile>>);

impl From<Vec<Vec<Tile>>> for State {
    fn from(v: Vec<Vec<Tile>>) -> State {
        State(v)
    }
}

impl State {

    fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        self.0.get(y)
            .map(|row| row.get(x).cloned())
            .flatten()
    }

    fn mutate<F>(&self, count_surrounding: F, comfort_threashold: usize) -> (State, bool)
    where F: Fn(&Self, usize, usize) -> usize {
        let mut new = vec![];
        let mut diff = false;
        for y in 0..self.0.len() {
            let row = &self.0[y];
            let mut new_row = vec![];
            for x in 0..row.len() {
                if let Some(t) = self.tile_at(x, y) {
                    match t {
                        Tile::Floor => new_row.push(Tile::Floor),
                        Tile::Occupied => {
                            let count = count_surrounding(self, x, y);
                            if count >= comfort_threashold {
                                new_row.push(Tile::EmptySeat);
                                diff = true;
                            } else {
                                new_row.push(Tile::Occupied);
                            }
                        },
                        Tile::EmptySeat => {
                            let count = count_surrounding(self, x, y);
                            if count == 0 {
                                new_row.push(Tile::Occupied);
                                diff = true;
                            } else {
                                new_row.push(Tile::EmptySeat);
                            }
                        }
                    }
                }
            }
            new.push(new_row);
        }
        (new.into(), diff)
    }

    fn stabalize<F>(self, count_surrounding: &F, comfort_threashold: usize) -> State
    where F: Fn(&Self, usize, usize) -> usize {
        let r: Result<State, State> = (0..).try_fold(self, |state, _| {
            let (state, mutated) = state.mutate(count_surrounding, comfort_threashold);
            if mutated {
                Ok(state)
            } else {
                Err(state)
            }
        });
        r.err().expect("unreachable!")
    }

    fn count_occupied(&self) -> usize {
        self.0.iter().map(|row| row.iter().filter(|t| t.is_occupied()).count()).sum()
    }
}

fn parse_row(line_no: usize, line: String) -> Result<Vec<Tile>, ReadError> {
    line.chars().map(|c| match c {
        'L' => Ok(Tile::EmptySeat),
        '#' => Ok(Tile::Occupied),
        '.' => Ok(Tile::Floor),
        _ => Err(ReadError::ParseError(line_no, format!("{}", c)))
    }).collect()
}

fn count_adjacent_part_1(state: &State, x: usize, y: usize) -> usize {
    let topleft = if x == 0 || y == 0 { 0 } else {
        state.tile_at(x - 1, y - 1).map(|t| t.into()).unwrap_or(0)
    };
    let topmiddle = if y == 0 { 0 } else {
        state.tile_at(x, y - 1).map(|t| t.into()).unwrap_or(0)
    };
    let topright = if y == 0 { 0 } else {
        state.tile_at(x + 1, y - 1).map(|t| t.into()).unwrap_or(0)
    };
    let left = if x == 0 { 0 } else {
        state.tile_at(x - 1, y).map(|t| t.into()).unwrap_or(0)
    };
    let right: usize = state.tile_at(x + 1, y).map(|t| t.into()).unwrap_or(0);
    let bottomleft = if x == 0 { 0 } else {
        state.tile_at(x - 1, y + 1).map(|t| t.into()).unwrap_or(0)
    };
    let bottommiddle: usize = state.tile_at(x, y + 1).map(|t| t.into()).unwrap_or(0);
    let bottomright: usize = state.tile_at(x + 1, y + 1).map(|t| t.into()).unwrap_or(0);
    topleft + topmiddle + topright + left + right + bottomleft + bottommiddle + bottomright
}

macro_rules! check {
    ($state:expr, $xiter:expr, $yiter:expr) => {
        $xiter.zip($yiter)
            .map(|(x, y)| $state.tile_at(x, y))
            .take_while(Option::is_some)
            .flat_map(|t| t)
            .find(|t| !t.is_floor())
            .map(|t| t.into())
            .unwrap_or(0);
    }
}

fn count_adjacent_part_2(state: &State, x: usize, y: usize) -> usize {
    let topleft = check!(state, (0..x).rev(), (0..y).rev());
    let topmiddle = check!(state, repeat(x), (0..y).rev());
    let topright = check!(state, x+1.., (0..y).rev());
    let left = check!(state, (0..x).rev(), repeat(y));
    let right = check!(state, x+1.., repeat(y));
    let bottomleft = check!(state, (0..x).rev(), y+1..);
    let bottommiddle = check!(state, repeat(x), y+1..);
    let bottomright = check!(state, x+1.., y+1..);
    topleft + topmiddle + topright + left + right + bottomleft + bottommiddle + bottomright
}

fn part1(state: State) -> usize {
    let state = state.stabalize(&count_adjacent_part_1, 4);
    state.count_occupied()
}

fn part2(state: State) -> usize {
    let state = state.stabalize(&count_adjacent_part_2, 5);
    state.count_occupied()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state: State = parse_input_file("day_11_input.txt", parse_row)?;
    let result = part1(state.clone());
    println!("part1: {}", result);
    let result = part2(state);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use adventofcode2020::parse_input_lines;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

        let state: State = parse_input_lines(input.as_bytes(), parse_row)?;

        assert_eq!(part1(state.clone()), 37);

        assert_eq!(part2(state.clone()), 26);

        Ok(())
    }
}
