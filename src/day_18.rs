use std::{fs, io::{BufRead, BufReader}};

#[derive(Clone, Copy, Debug)]
enum Op {
    Num(i64),
    Add,
    Mul,
    Push,
    Pop
}

fn parse_math(line: &str) -> Vec<Op> {
    line.chars().filter_map(|c| match c {
        '0'..='9' => Some(Op::Num(c as i64 - '0' as i64)),
        '+' => Some(Op::Add),
        '*' => Some(Op::Mul),
        '(' => Some(Op::Push),
        ')' => Some(Op::Pop),
        ' ' => None,
        _ => panic!("invalid input")
    }).collect()
}

fn process_math_part1(math: &mut dyn Iterator<Item=&Op>) -> i64 {
    let mut result: Option<i64> = None;
    let mut next_op: Option<Op> = None;
    loop {
        match math.next() {
            Some(&Op::Num(val)) => {
                result = Some(match result {
                    Some(c) => match next_op {
                        Some(Op::Add) => c + val,
                        Some(Op::Mul) => c * val,
                        _ => panic!("invalid state"),
                    },
                    None => val
                });
            },
            op @ Some(&Op::Add) | op @ Some(&Op::Mul) => {
                next_op = op.cloned();
            },
            Some(&Op::Push) => {
                let val = process_math_part1(math);
                result = Some(match result {
                    Some(c) => match next_op {
                        Some(Op::Add) => c + val,
                        Some(Op::Mul) => c * val,
                        _ => panic!("invalid state"),
                    },
                    None => val
                });
            },
            None | Some(&Op::Pop) => {
                break
            }
        };
    }
    result.unwrap()
}

fn process_math_part2(math: &mut dyn Iterator<Item=&Op>) -> i64 {
    let mut current_op: Option<&Op> = None;
    let mut current_vals: Vec<i64> = vec![];
    let mut floating_val: Option<i64> = None;
    let mut stack: Vec<(Option<&Op>, Vec<i64>)> = vec![];
    loop {
        match math.next() {
            Some(&Op::Num(val)) => {
                if let Some(fv) = floating_val {
                    current_vals.push(fv);
                }
                floating_val = Some(val);
            },
            Some(op @ &Op::Add) | Some(op @ &Op::Mul) => {
                match current_op {
                    None => {
                        current_op = Some(op);
                    },
                    Some(cop) => {
                        match (cop, op) {
                            (Op::Add, Op::Mul) => {
                                current_op = Some(op);
                                current_vals.push(
                                    floating_val.take().unwrap()
                                );
                                floating_val = Some(
                                    current_vals.iter().sum()
                                );
                                current_vals = vec![];
                            },
                            (Op::Mul, Op::Add) => {
                                stack.push(
                                    (Some(cop), current_vals.clone())
                                );
                                current_op = Some(op);
                                current_vals = vec![];
                            },
                            _ => {}
                        }
                    },
                }
            },
            Some(&Op::Push) => {
                if let Some(fv) = floating_val.take() {
                    current_vals.push(fv);
                }
                floating_val = Some(process_math_part2(math));
            },
            None | Some(&Op::Pop) => {
                loop {
                    if let Some(fv) = floating_val.take() {
                        current_vals.push(fv);
                    }
                    floating_val = match current_op {
                        Some(Op::Add) => {
                            Some(current_vals.iter().sum())
                        },
                        Some(Op::Mul) => {
                            Some(current_vals.iter().product())
                        },
                        _ => {
                            if current_vals.len() == 1 {
                                current_vals.pop()
                            } else {
                                dbg!(&current_vals);
                                None
                            }
                        }
                    };

                    if let Some((op, vals)) = stack.pop() {
                        current_op = op;
                        current_vals = vals;
                    } else {
                        return floating_val.unwrap();
                    }
                }
            }
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open("day_18_input.txt")?;
    let reader = BufReader::new(f);
    let math = reader.lines().filter_map(Result::ok)
        .map(|line| parse_math(&line))
        .collect::<Vec<_>>();
    let result: i64 = math.iter().map(|math| process_math_part1(&mut math.iter()))
        .sum();
    println!("part1: {}", result);

    let result: i64 = math.iter().map(|math| process_math_part2(&mut math.iter()))
        .sum();
    println!("part2: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let math = parse_math("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(process_math_part1(&mut math.iter()), 71);
        assert_eq!(process_math_part1(&mut parse_math("1 + (2 * 3) + (4 * (5 + 6))").iter()), 51);
        assert_eq!(process_math_part1(&mut parse_math("2 * 3 + (4 * 5)").iter()), 26);
        assert_eq!(process_math_part1(&mut parse_math("5 + (8 * 3 + 9 + 3 * 4 * 3)").iter()), 437);
        assert_eq!(process_math_part1(&mut parse_math("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").iter()), 12240);
        assert_eq!(process_math_part1(&mut parse_math("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").iter()), 13632);
    }

    #[test]
    fn test_2() {
        assert_eq!(process_math_part2(&mut parse_math("1 + 2 * 3 + 4 * 5 + 6").iter()), 231);
        assert_eq!(process_math_part2(&mut parse_math("1 + (2 * 3) + (4 * (5 + 6))").iter()), 51);
        assert_eq!(process_math_part2(&mut parse_math("2 * 3 + (4 * 5)").iter()), 46);
        assert_eq!(process_math_part2(&mut parse_math("5 + (8 * 3 + 9 + 3 * 4 * 3)").iter()), 1445);
        assert_eq!(process_math_part2(&mut parse_math("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").iter()), 669060);
        assert_eq!(process_math_part2(&mut parse_math("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").iter()), 23340);
    }
}
