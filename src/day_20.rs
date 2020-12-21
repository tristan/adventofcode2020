use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Tile {
    id: usize,
    rows: Vec<u16>,
    n: u16,
    e: u16,
    s: u16,
    w: u16,
    nn: Option<usize>,
    ne: Option<usize>,
    ns: Option<usize>,
    nw: Option<usize>
}

const TILE_DIM: usize = 10;
const REV_SHIFT: usize = 6;

fn rev(val: u16) -> u16 {
    val.reverse_bits() >> REV_SHIFT
}

fn build_tile(id: usize, rows: Vec<u16>) -> Tile {
    let n = rows[0];
    let s = rev(rows[TILE_DIM - 1]);
    let (w, e) = rows.iter().fold((0, 0), |(w, e), row| (
        (w << 1) + (row >> (TILE_DIM - 1)), (e << 1) + (row & 1)
    ));
    let w = rev(w);
    Tile::new(id, rows, n, e, s, w)
}

fn push_tile(tiles: &mut Vec<Tile>, id: usize, rows: Vec<u16>) -> Vec<u16> {
    tiles.push(build_tile(id, rows));
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
        Tile { id, rows, n, e, s, w, nn: None, ne: None, ns: None, nw: None }
    }

    // rotates the tile 90 degrees clockwise
    fn rotate(&self, amount: usize) -> Tile {
        let amount = amount % 4;
        if amount == 0 {
            self.clone()
        } else if amount == 1 {
            let mut rows = vec![0; TILE_DIM];
            self.rows.iter().for_each(|row| {
                (0..TILE_DIM).for_each(|i| {
                    rows[i] = (rows[i] << 1) + ((row >> i) & 1)
                })
            });
            let mut t = build_tile(self.id, rows.into_iter().map(rev).rev().collect());
            t.nn = self.nw;
            t.ne = self.nn;
            t.ns = self.ne;
            t.nw = self.ns;
            t
        } else if amount == 2 {
            let rows = self.rows.iter().map(|&r| rev(r)).rev().collect();
            let mut t = build_tile(self.id, rows);
            t.nn = self.ns;
            t.ne = self.nw;
            t.ns = self.nn;
            t.nw = self.ne;
            t
        } else if amount == 3 {
            let mut rows = vec![0; TILE_DIM];
            self.rows.iter().for_each(|row| {
                (0..TILE_DIM).for_each(|i| {
                    rows[i] = (rows[i] << 1) + ((row >> i) & 1)
                })
            });
            let mut t = build_tile(self.id, rows);
            t.nn = self.ne;
            t.ne = self.ns;
            t.ns = self.nw;
            t.nw = self.nn;
            t
        } else {
            unreachable!();
        }
    }

    fn flip_ns(&self) -> Tile {
        let mut t = build_tile(
            self.id,
            self.rows.iter().rev().cloned().collect()
        );
        t.nn = self.ns;
        t.ne = self.ne;
        t.ns = self.nn;
        t.nw = self.nw;
        t
    }

    fn flip_ew(&self) -> Tile {
        let mut t = build_tile(
            self.id,
            self.rows.iter()
                .map(|&row| rev(row))
                .collect()
        );
        t.nn = self.nn;
        t.ne = self.nw;
        t.ns = self.ns;
        t.nw = self.ne;
        t
    }

    fn all_values(&self) -> Vec<(u16, usize)> {
        vec![
            (self.n, 0),
            (self.e, 1),
            (self.s, 2),
            (self.w, 3),
            (rev(self.n), 4),
            (rev(self.e), 5),
            (rev(self.s), 6),
            (rev(self.w), 7),
        ]
    }
}

fn rotate_field(orig: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut field: Vec<Vec<u8>> = vec![vec![0; orig.len()]; orig.len()];
    orig.iter().enumerate().for_each(|(oy, row)| {
        row.iter().enumerate().for_each(|(ox, &ov)| {
            let y = ox;
            let x = (orig.len() - 1) - oy;
            field[y][x] = ov;
        });
    });
    field
}

fn flip_field(orig: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    orig.into_iter().rev().collect()
}

fn count_sea_monsters(field: &Vec<Vec<u8>>) -> usize {
    //   01234567890123456789
    // 0                   #
    // 1 #    ##    ##    ###
    // 2  #  #  #  #  #  #
    (0..(field.len() - 3)).map(|y| {
        (0..field.len() - 17).filter(|&x| {
            let is_monster =
                field[y+1][x] == 1 &&
                field[y+2][x+1] == 1 &&
                field[y+2][x+4] == 1 &&
                field[y+1][x+5] == 1 &&
                field[y+1][x+6] == 1 &&
                field[y+2][x+7] == 1 &&
                field[y+2][x+10] == 1 &&
                field[y+1][x+11] == 1 &&
                field[y+1][x+12] == 1 &&
                field[y+2][x+13] == 1 &&
                field[y+2][x+16] == 1 &&
                field[y+1][x+17] == 1 &&
                field[y][x+18] == 1 &&
                field[y+1][x+18] == 1 &&
                field[y+1][x+19] == 1
                ;
            if is_monster {
                println!("MONSTER AT: {},{}", x,y);
            }
            is_monster
        }).count()
    }).sum()
}

fn print_field(field: &Vec<Vec<u8>>) {
    for row in field {
        for pos in row {
            match pos {
                0 => print!("."),
                1 => print!("#"),
                _ => print!(" "),
            }
        }
        println!();
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.rows.iter().enumerate().map(|(r, row)| {
            let (prefix, suffix): (String, String) = if r == 4 {
                (
                    if self.nw.is_some() { format!("{} ", self.nw.unwrap()) } else { "     ".to_string() },
                    if self.ne.is_some() { format!(" {}\n", self.ne.unwrap()) } else { "\n".to_string() }
                )
            } else if r == 6 {
                (
                    format!("{:4} ", self.w),
                    format!(" {:4}\n", self.e)
                )
            } else {
                ("     ".to_string(), "\n".to_string())
            };
            let middle = (0..TILE_DIM).rev().map(|i| match (row >> i) & 1 {
                0 => '.',
                1 => '#',
                _ => unreachable!()
            }).collect::<String>();
            [prefix, middle, suffix].join("")
        }).collect::<String>();
        let nn = if let Some(nn) = self.nn {
            format!("{:4}", nn)
        } else { "    ".to_owned() } ;
        let ns = if let Some(ns) = self.ns {
            format!("{:4}", ns)
        } else { "    ".to_owned() } ;
        write!(f, "Tile: {}\n     {}  {:4}\n{}     {}  {:4}",
               self.id, nn, self.n,
               s,
               ns, self.s
        )
    }
}

fn part1(tiles: &[Tile]) -> usize {
    let mut set: HashMap<u16, Vec<(usize, usize)>> = HashMap::new();
    tiles.into_iter().for_each(|tile| {
        tile.all_values()
            .into_iter()
            .for_each(|(v, d)| {
                if let Some(o) = set.get_mut(&v) {
                    if o.len() == 2 {
                        panic!("{} = {:?}", v, o);
                    }
                    o.push((tile.id, d));
                } else {
                    set.insert(v, vec![(tile.id, d)]);
                };
            })
    });
    let mut edges: HashMap<usize, usize> = HashMap::new();
    set.iter().for_each(|(_, tiles)| if tiles.len() == 2 {
        tiles.iter().for_each(|(id, _)| {
            edges.insert(*id, match edges.get(id) {
                Some(o) => o + 1,
                None => 1
            });
        })
    });
    edges.iter().filter_map(|(id, &edge_count)| if edge_count == 4 {
        Some(id)
    } else {
        None
    }).product()
}

fn part2(tiles: &[Tile]) -> usize {
    let mut set: HashMap<u16, Vec<(usize, usize)>> = HashMap::new();
    tiles.into_iter().for_each(|tile| {
        tile.all_values()
            .into_iter()
            .for_each(|(v, e)| {
                if let Some(o) = set.get_mut(&v) {
                    if o.len() == 2 {
                        panic!("{} = {:?}", v, o);
                    }
                    o.push((tile.id, e));
                } else {
                    set.insert(v, vec![(tile.id, e)]);
                };
            })
    });
    let mut tiles: HashMap<usize, Tile> = tiles.iter().map(|t| (t.id, t.clone())).collect();
    set.iter().for_each(|(_, ts)| if ts.len() == 2 {
        let &(t1, e1) = &ts[0];
        let &(t2, e2) = &ts[1];
        {
            let mut t1 = tiles.get_mut(&t1).unwrap();
            if e1 == 0 || e1 == 4 {
                t1.nn = Some(t2);
            } else if e1 == 1 || e1 == 5 {
                t1.ne = Some(t2);
            } else if e1 == 2 || e1 == 6 {
                t1.ns = Some(t2);
            } else if e1 == 3 || e1 == 7 {
                t1.nw = Some(t2);
            }
        }
        {
            let mut t2 = tiles.get_mut(&t2).unwrap();
            if e2 == 0 || e2 == 4 {
                t2.nn = Some(t1);
            } else if e2 == 1 || e2 == 5 {
                t2.ne = Some(t1);
            } else if e2 == 2 || e2 == 6 {
                t2.ns = Some(t1);
            } else if e2 == 3 || e2 == 7 {
                t2.nw = Some(t1);
            }
        }
    });

    let corner_id = tiles.iter().find_map(|(_, tile)| {
        let edges = if tile.nn.is_some() { 1 } else { 0 }
        + if tile.ne.is_some() { 1 } else { 0 }
        + if tile.ns.is_some() { 1 } else { 0 }
        + if tile.nw.is_some() { 1 } else { 0 };
        if edges == 2 {
            Some(tile.id)
        } else {
            None
        }
    }).unwrap();

    let corner_tile = tiles.remove(&corner_id).unwrap();
    // orient to the top left corner
    let corner_tile = if corner_tile.nn.is_some() && corner_tile.ne.is_some() {
        corner_tile.rotate(1)
    } else if corner_tile.ne.is_some() && corner_tile.ns.is_some() {
        corner_tile
    } else if corner_tile.ns.is_some() && corner_tile.nw.is_some() {
        corner_tile.rotate(3)
    } else if corner_tile.nw.is_some() && corner_tile.nn.is_some() {
        corner_tile.rotate(2)
    } else {
        panic!("no edges set for corner tile!");
    };
    let mut rows: Vec<Vec<Tile>> = vec![];
    let mut row: Vec<Tile> = vec![corner_tile];

    loop {
        let (prev_tile_ne, prev_tile_e) = {
            let tile = &row[row.len() - 1];
            (tile.ne, tile.e)
        };
        if let Some(next_tile_id) = prev_tile_ne {
            let next_tile = tiles.remove(&next_tile_id).unwrap();
            // find rotation that matches prev tile's edge
            let next_tile = if prev_tile_e == rev(next_tile.n) {
                // rotate so n is w
                next_tile.rotate(3)
            } else if prev_tile_e == rev(next_tile.e) {
                // rotate so e is w
                next_tile.rotate(2)
            } else if prev_tile_e == rev(next_tile.s) {
                // rotate so s is w
                next_tile.rotate(1)
            } else if prev_tile_e == rev(next_tile.w) {
                next_tile
            } else if prev_tile_e == next_tile.n {
                // rotate then flip
                next_tile.rotate(3).flip_ns()
            } else if prev_tile_e == next_tile.e {
                next_tile.rotate(2).flip_ns()
            } else if prev_tile_e == next_tile.s {
                next_tile.rotate(1).flip_ns()
            } else if prev_tile_e == next_tile.w {
                next_tile.flip_ns()
            } else {
                panic!("didn't find match between tiles")
            };
            row.push(next_tile);
        } else {
            // push row
            rows.push(row);
            row = vec![];
            // find ns edge (or break if none)
            let (prev_tile_ns, prev_tile_s) = {
                let tile = &rows[rows.len() - 1][0];
                (tile.ns, tile.s)
            };
            if let Some(next_tile_id) = prev_tile_ns {
                let next_tile = tiles.remove(&next_tile_id).unwrap();
                // find rotation that matches prev tile's edge
                if prev_tile_s == rev(next_tile.n) {
                    row.push(next_tile);
                } else if prev_tile_s == rev(next_tile.e) {
                    // rotate so e is n
                    row.push(next_tile.rotate(3));
                } else if prev_tile_s == rev(next_tile.s) {
                    // rotate so s is n
                    row.push(next_tile.rotate(2));
                } else if prev_tile_s == rev(next_tile.w) {
                    // rotate so w is n
                    row.push(next_tile.rotate(1));
                } else if prev_tile_s == next_tile.n {
                    // flip ew
                    row.push(next_tile.flip_ew());
                } else if prev_tile_s == next_tile.e {
                    row.push(next_tile.flip_ns().rotate(3));
                } else if prev_tile_s == next_tile.s {
                    row.push(next_tile.flip_ew().rotate(2));
                } else if prev_tile_s == next_tile.w {
                    row.push(next_tile.flip_ns().rotate(1));
                } else {
                    panic!("didn't find match between tiles")
                }
            } else {
                break;
            }
        }
    }

    let mut field: Vec<Vec<u8>> = vec![
        vec![0; rows.len() * (TILE_DIM - 2)];
        rows.len() * (TILE_DIM - 2)
    ];
    rows.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, tile)| {
            tile.rows.iter().skip(1).take(TILE_DIM - 2).enumerate()
                .for_each(|(k, tile_row)| {
                    (1..(TILE_DIM-1)).rev().enumerate().for_each(|(o, shift)| {
                        let v = ((tile_row >> shift) & 1) as u8;
                        let y = i * (TILE_DIM - 2) + k;
                        let x = j * (TILE_DIM - 2) + o;
                        field[y][x] = v;
                    })
                });
        });
    });

    let mut max_sea_monsters = 0;
    for i in 0..8 {
        let sea_monsters = count_sea_monsters(&field);
        println!("{}: {}", i, sea_monsters);
        if sea_monsters > 0 {
            print_field(&field);
        }
        max_sea_monsters = max_sea_monsters.max(sea_monsters);
        field = rotate_field(field);
        if i == 3 {
            field = flip_field(field);
        }
    }

    field.iter()
        .map(|row| row.iter().filter(|&&v| v == 1).count())
        .sum::<usize>()
        - (max_sea_monsters * 15)
}

fn main() {
    let f = File::open("day_20_input.txt").unwrap();
    let tiles = parse_input(BufReader::new(f));
    let result = part1(&tiles);
    println!("part1: {}", result);
    let result = part2(&tiles);
    println!("part2: {}", result);
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
        assert_eq!(part2(&tiles), 273);
    }

    #[test]
    fn test_rotate() {
        let rows = vec![
            0b1010111110,
            0b0100111111,
            0b0010000000,
            0b1111110000,
            0b1111010010,
            0b0100010110,
            0b1011111011,
            0b0010111000,
            0b0010000000,
            0b0010111000,
        ];
        let t = build_tile(0, rows);
        assert_eq!(t.n, 0b1010111110);
        assert_eq!(t.e, 0b0100001000);
        assert_eq!(t.s, 0b0001110100);
        assert_eq!(t.w, 0b0001011001);
        assert_eq!(t.all_values(), vec![
            (0b1010111110, 0),
            (0b0100001000, 1),
            (0b0001110100, 2),
            (0b0001011001, 3),
            (0b0111110101, 4),
            (0b0001000010, 5),
            (0b0010111000, 6),
            (0b1001101000, 7),
        ]);

        let t1 = t.rotate(1);
        assert_eq!(t1.rows, vec![
            0b0001011001,
            0b0000111010,
            0b1111011101,
            0b0001011000,
            0b1011001011,
            0b1011111011,
            0b1011000011,
            0b0000100011,
            0b0001110011,
            0b0001000010,
        ]);
        assert_eq!(t1.n, t.w);
        assert_eq!(t1.s, t.e);
        assert_eq!(t1.e, t.n);
        assert_eq!(t1.w, t.s);

        let t2 = t.rotate(2);
        assert_eq!(t2.rows, vec![
            0b0001110100,
            0b0000000100,
            0b0001110100,
            0b1101111101,
            0b0110100010,
            0b0100101111,
            0b0000111111,
            0b0000000100,
            0b1111110010,
            0b0111110101,
        ]);
        assert_eq!(t2.n, t.s);
        assert_eq!(t2.s, t.n);
        assert_eq!(t2.e, t.w);
        assert_eq!(t2.w, t.e);

        let t3 = t.rotate(3);
        assert_eq!(t3.rows, vec![
            0b0100001000,
            0b1100111000,
            0b1100010000,
            0b1100001101,
            0b1101111101,
            0b1101001101,
            0b0001101000,
            0b1011101111,
            0b0101110000,
            0b1001101000,
        ]);
        assert_eq!(t3.n, t.e);
        assert_eq!(t3.s, t.w);
        assert_eq!(t3.e, t.s);
        assert_eq!(t3.w, t.n);

        let fns = t.flip_ns();
        assert_eq!(fns.rows, vec![
            0b0010111000,
            0b0010000000,
            0b0010111000,
            0b1011111011,
            0b0100010110,
            0b1111010010,
            0b1111110000,
            0b0010000000,
            0b0100111111,
            0b1010111110,
        ]);
        assert_eq!(fns.n, t.s.reverse_bits() >> REV_SHIFT);
        assert_eq!(fns.s, t.n.reverse_bits() >> REV_SHIFT);
        assert_eq!(fns.e, t.e.reverse_bits() >> REV_SHIFT);
        assert_eq!(fns.w, t.w.reverse_bits() >> REV_SHIFT);
        let few = t.flip_ew();
        assert_eq!(few.rows, vec![
            0b0111110101,
            0b1111110010,
            0b0000000100,
            0b0000111111,
            0b0100101111,
            0b0110100010,
            0b1101111101,
            0b0001110100,
            0b0000000100,
            0b0001110100,
        ]);
        assert_eq!(few.s, t.s.reverse_bits() >> REV_SHIFT);
        assert_eq!(few.n, t.n.reverse_bits() >> REV_SHIFT);
        assert_eq!(few.w, t.e.reverse_bits() >> REV_SHIFT);
        assert_eq!(few.e, t.w.reverse_bits() >> REV_SHIFT);
    }
}
