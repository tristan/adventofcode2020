use std::fs;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .fold(HashSet::new(), |mut set, c| {
                    set.insert(c);
                    set
                })
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input.trim().split("\n\n")
        .map(|group| {
            group.split("\n")
                .map(|row| {
                    row
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .fold(HashSet::new(), |mut set, c| {
                            set.insert(c);
                            set
                        })
                })
                .fold(None, |acc, set| {
                    match acc {
                        None => Some(set),
                        Some(acc) => {
                            Some(acc.intersection(&set).cloned().collect())
                        }
                    }
                })
                .map(|set| set.len())
                .unwrap_or(0)
        })
        .sum()
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("day_06_input.txt")?;
    let result = part1(&input);
    println!("part1: {}", result);
    let result = part2(&input);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parts() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        assert_eq!(part1(input), 11);
        assert_eq!(part2(input), 6);

    }
}
