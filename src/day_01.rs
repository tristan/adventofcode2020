use adventofcode2020::read_input_lines;

fn part1(input: &[u64]) -> Option<u64> {
    input.iter().enumerate().find_map(|(i, lhs)| {
        input.iter().enumerate().find_map(|(j, rhs)| {
            if i == j {
                None
            } else {
                if lhs + rhs == 2020 {
                    Some(lhs * rhs)
                } else {
                    None
                }
            }
        })
    })
}

fn part2(input: &[u64]) -> Option<u64> {
    input.iter().enumerate().find_map(|(i, lhs)| {
        input.iter().enumerate().find_map(|(j, rhs)| {
            input.iter().enumerate().find_map(|(k, xhs)| {
                if i == j || i == k || j == k {
                    None
                } else {
                    if lhs + rhs + xhs == 2020 {
                        Some(lhs * rhs * xhs)
                    } else {
                        None
                    }
                }
            })
        })
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_lines("day_01_input.txt")?;
    let result = part1(&input).unwrap();
    println!("part1: {}", result);
    let result = part2(&input).unwrap();
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn std::error::Error>> {
        let input = [
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        assert_eq!(part1(&input), Some(514579));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn std::error::Error>> {
        let input = [
            1721,
            979,
            366,
            299,
            675,
            1456,
        ];
        assert_eq!(part2(&input), Some(241861950));
        Ok(())
    }
}
