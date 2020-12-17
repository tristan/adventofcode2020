use std::{fs, io::{self, BufRead, BufReader}};
use std::cmp::min;

#[derive(Clone, PartialEq)]
enum Cube {
    Active,
    Inactive
}

#[derive(Clone)]
struct PocketDimension3d {
    cubes: Vec<Vec<Vec<Cube>>>
}

impl PocketDimension3d {
    fn from_file(filename: &str) -> io::Result<PocketDimension3d> {
        let f = fs::File::open(filename)?;
        Ok(PocketDimension3d::from_reader(BufReader::new(f)))
    }

    fn from_reader<R>(reader: R) -> PocketDimension3d
    where R: BufRead {
        let cubes = reader.lines()
            .filter_map(Result::ok)
            .filter(|line| line.len() > 0)
            .map(|line| line.chars().map(|c| match c {
                '#' => Cube::Active,
                '.' => Cube::Inactive,
                _ => panic!("Invalid cube state")
            }).collect())
            .collect::<Vec<Vec<_>>>();
        let h = cubes.len();
        let w = cubes[0].len();
        PocketDimension3d {
            cubes: vec![vec![vec![Cube::Inactive; w]; h], cubes, vec![vec![Cube::Inactive; w]; h]]
        }
    }

    fn cycle(&mut self) {
        let d = self.cubes.len() + 2;
        let h = self.cubes[0].len() + 2;
        let w = self.cubes[0][0].len() + 2;
        self.cubes.iter_mut()
            .for_each(|slice| {
                slice.iter_mut()
                    .for_each(|row| {
                        row.push(Cube::Inactive);
                        row.push(Cube::Inactive);
                        row.rotate_right(1);
                    });
                slice.push(vec![Cube::Inactive; w]);
                slice.push(vec![Cube::Inactive; w]);
                slice.rotate_right(1);
            });
        self.cubes.push(vec![vec![Cube::Inactive; w]; h]);
        self.cubes.push(vec![vec![Cube::Inactive; w]; h]);
        self.cubes.rotate_right(1);

        self.cubes = (0..d).map(|z| {
            (0..h).map(|y| {
                (0..w).map(|x| {
                    let active: usize = (z.saturating_sub(1)..=min(z+1, d-1)).map(|dz| -> usize {
                        (y.saturating_sub(1)..=min(y+1, h-1)).map(|dy| {
                            (x.saturating_sub(1)..=min(x+1, w-1)).filter_map(|dx| {
                                if dx == x && dy == y && dz == z {
                                    None
                                } else {
                                    if self.cubes[dz][dy][dx] == Cube::Active {
                                        Some(())
                                    } else {
                                        None
                                    }
                                }
                            }).count()
                        }).sum()
                    }).sum();
                    match self.cubes[z][y][x] {
                        Cube::Active => {
                            if active == 2 || active == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        },
                        Cube::Inactive => {
                            if active == 3 {
                                Cube::Active
                            } else {
                                Cube::Inactive
                            }
                        }
                    }
                }).collect()
            }).collect()
        }).collect();
    }

    fn count_active(&self) -> usize {
        self.cubes.iter().map(|slice| -> usize {
            slice.iter().map(|row| {
                row.iter().filter(|c| c == &&Cube::Active).count()
            }).sum()
        }).sum()
    }

    fn print(&self) {
        for (z, slice) in self.cubes.iter().enumerate() {
            println!("z = {}", (z as isize) - (self.cubes.len() as isize / 2));
            for row in slice.iter() {
                for cube in row {
                    match cube {
                        Cube::Active => print!("#"),
                        Cube::Inactive => print!(".")
                    }
                }
                println!("");
            }
            println!("");
        }
    }
}

#[derive(Clone)]
struct PocketDimension4d {
    cubes: Vec<Vec<Vec<Vec<Cube>>>>
}

impl PocketDimension4d {
    fn from_file(filename: &str) -> io::Result<PocketDimension4d> {
        let f = fs::File::open(filename)?;
        Ok(PocketDimension4d::from_reader(BufReader::new(f)))
    }

    fn from_reader<R>(reader: R) -> PocketDimension4d
    where R: BufRead {
        let cubes = PocketDimension3d::from_reader(reader).cubes;
        let d = cubes.len();
        let h = cubes[0].len();
        let w = cubes[0][0].len();

        let w_0 = vec![vec![vec![Cube::Inactive; w]; h]; d];
        let w_n = vec![vec![vec![Cube::Inactive; w]; h]; d];

        PocketDimension4d {
            cubes: vec![w_0, cubes, w_n]
        }
    }

    fn cycle(&mut self) {
        let wat = self.cubes.len() + 2;
        let depth = self.cubes[0].len() + 2;
        let height = self.cubes[0][0].len() + 2;
        let width = self.cubes[0][0][0].len() + 2;

        self.cubes.iter_mut()
            .for_each(|time| {
                time.iter_mut().for_each(|slice| {
                    slice.iter_mut()
                        .for_each(|row| {
                            row.push(Cube::Inactive);
                            row.push(Cube::Inactive);
                            row.rotate_right(1);
                        });
                    slice.push(vec![Cube::Inactive; width]);
                    slice.push(vec![Cube::Inactive; width]);
                    slice.rotate_right(1);
                });
                time.push(vec![vec![Cube::Inactive; width]; height]);
                time.push(vec![vec![Cube::Inactive; width]; height]);
                time.rotate_right(1);
            });
        self.cubes.push(vec![vec![vec![Cube::Inactive; width]; height]; depth]);
        self.cubes.push(vec![vec![vec![Cube::Inactive; width]; height]; depth]);
        self.cubes.rotate_right(1);

        self.cubes = (0..wat).map(|w| {
            (0..depth).map(|z| {
                (0..height).map(|y| {
                    (0..width).map(|x| {
                        let active: usize = (w.saturating_sub(1)..=min(w+1, wat-1)).map(|dw| -> usize {
                            (z.saturating_sub(1)..=min(z+1, depth-1)).map(|dz| -> usize {
                                (y.saturating_sub(1)..=min(y+1, height-1)).map(|dy| {
                                    (x.saturating_sub(1)..=min(x+1, width-1)).filter_map(|dx| {
                                        if dx == x && dy == y && dz == z && dw == w {
                                            None
                                        } else {
                                            if self.cubes[dw][dz][dy][dx] == Cube::Active {
                                                Some(())
                                            } else {
                                                None
                                            }
                                        }
                                    }).count()
                                }).sum()
                            }).sum()
                        }).sum();
                        match self.cubes[w][z][y][x] {
                            Cube::Active => {
                                if active == 2 || active == 3 {
                                    Cube::Active
                                } else {
                                    Cube::Inactive
                                }
                            },
                            Cube::Inactive => {
                                if active == 3 {
                                    Cube::Active
                                } else {
                                    Cube::Inactive
                                }
                            }
                        }
                    }).collect()
                }).collect()
            }).collect()
        }).collect();
    }

    fn count_active(&self) -> usize {
        self.cubes.iter().map(|time| -> usize {
            time.iter().map(|slice| -> usize {
                slice.iter().map(|row| {
                    row.iter().filter(|c| c == &&Cube::Active).count()
                }).sum()
            }).sum()
        }).sum()
    }
}

fn part1(mut pd: PocketDimension3d) -> usize {
    (0..6).for_each(|_| pd.cycle());
    pd.count_active()
}


fn part2(mut pd: PocketDimension4d) -> usize {
    (0..6).for_each(|_| pd.cycle());
    pd.count_active()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pd = PocketDimension3d::from_file("day_17_input.txt")?;
    let result = part1(pd);
    println!("part1: {}", result);
    let pd = PocketDimension4d::from_file("day_17_input.txt")?;
    let result = part2(pd);
    println!("part2: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = r#".#.
..#
###
"#;
        let pd = PocketDimension3d::from_reader(input.as_bytes());
        assert_eq!(part1(pd), 112);

        let pd = PocketDimension4d::from_reader(input.as_bytes());
        assert_eq!(part2(pd), 848);
    }

}
