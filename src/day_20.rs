use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

struct Tile {
    id: usize,
    rows: Vec<u16>,
    n: u16,
    e: u16,
    s: u16,
    w: u16
}

const TILE_DIM: usize = 10;
const REV_SHIFT: usize = 6;

fn push_tile(tiles: &mut Vec<Tile>, id: usize, rows: Vec<u16>) -> Vec<u16> {
    let n = rows[0];
    let s = rows[TILE_DIM - 1].reverse_bits() >> REV_SHIFT;
    let (w, e) = rows.iter().fold((0, 0), |(w, e), row| (
        (w << 1) + (row >> (TILE_DIM - 1)), (e << 1) + (row & 1)
    ));
    let w = w.reverse_bits() >> REV_SHIFT;
    tiles.push(Tile::new(id, rows, n, e, s, w));
    vec![]
}

fn parse_input<R: BufRead>(reader: R) -> Vec<Tile> {
    let mut tiles = vec![];
    let mut reader = reader.lines().filter_map(Result::ok);
    let mut tile: Vec<u16> = vec![];
    let mut id = 0;

    while let Some(line) = reader.next() {
        if line.len() == 0 {
            tile = push_tile(&mut tiles, id, tile);
            id = 0;
        } else if id == 0 {
            id = line[5..line.len()-1].parse().expect("invalid id");
        } else {
            let row = line.chars().fold(0, |a, c| (a << 1) + match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("invalid tile row"),
            });
            tile.push(row);
        }
    }
    push_tile(&mut tiles, id, tile);
    tiles
}

impl Tile {
    fn new(id: usize, rows: Vec<u16>, n: u16, e: u16, s: u16, w: u16) -> Tile {
        Tile { id, rows, n, e, s, w }
    }

    // rotates the tile 90 degrees clockwise
    fn rotate(&self) -> Tile {
        // TODO: if needed, keep track of transformations
        Tile::new(self.id, vec![], self.w, self.n, self.e, self.s)
    }

    fn flip_ns(&self) -> Tile {
        Tile::new(
            self.id, vec![],
            self.s.reverse_bits() >> REV_SHIFT,
            self.e.reverse_bits() >> REV_SHIFT,
            self.n.reverse_bits() >> REV_SHIFT,
            self.w.reverse_bits() >> REV_SHIFT
        )
    }

    fn flip_ew(&self) -> Tile {
        Tile::new(
            self.id, vec![],
            self.n.reverse_bits() >> REV_SHIFT,
            self.w.reverse_bits() >> REV_SHIFT,
            self.s.reverse_bits() >> REV_SHIFT,
            self.e.reverse_bits() >> REV_SHIFT
        )
    }

    fn all_values(&self) -> HashSet<u16> {
        let mut result = HashSet::new();
        result.insert(self.n);
        result.insert(self.s);
        result.insert(self.e);
        result.insert(self.w);

        let x = self.flip_ns();
        result.insert(x.n);
        result.insert(x.s);
        result.insert(x.e);
        result.insert(x.w);

        let x = self.flip_ew();
        result.insert(x.n);
        result.insert(x.s);
        result.insert(x.e);
        result.insert(x.w);

        result
    }
}

fn part1(tiles: &[Tile]) -> usize {
    let mut set: HashMap<u16, Vec<usize>> = HashMap::new();
    tiles.into_iter().for_each(|tile| {
        tile.all_values()
            .into_iter()
            .for_each(|v| {
                if let Some(o) = set.get_mut(&v) {
                    if o.len() == 2 {
                        panic!("{} = {:?}", v, o);
                    }
                    o.push(tile.id);
                } else {
                    set.insert(v, vec![tile.id]);
                };
            })
    });
    let mut edges: HashMap<usize, usize> = HashMap::new();
    set.iter().for_each(|(_, tiles)| if tiles.len() == 2 {
        tiles.iter().for_each(|id| {
            edges.insert(*id, match edges.get(id) {
                Some(o) => o + 1,
                None => 1
            });
        })
    });
    dbg!(&edges);
    edges.iter().filter_map(|(id, &edge_count)| if edge_count == 4 {
        Some(id)
    } else {
        None
    }).product()
}

fn main() {
    let f = File::open("day_20_input.txt").unwrap();
    let tiles = parse_input(BufReader::new(f));
    let result = part1(&tiles);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let f = File::open("day_20_test_input.txt").unwrap();
        let tiles = parse_input(BufReader::new(f));
        assert_eq!(tiles.len(), 9);
        assert_eq!(part1(&tiles), 20899048083289);

    }

}
