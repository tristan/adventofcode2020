use adventofcode2020::read_input_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_lines("day_09_input.txt")?;
    let result = part1(&input, 25);
    println!("part1: {}", result);
    let result = part2(&input, result);
    println!("part2: {}", result);
    Ok(())
}

fn part1(input: &[u64], preamble: usize) -> u64 {
    let choices = &input[..preamble];
    let target = input[preamble];
    let m = choices.iter().enumerate().any(|(i, a)| {
        choices.iter().enumerate().any(|(j, b)| {
            i != j && a + b == target
        })
    });
    if m {
        part1(&input[1..], preamble)
    } else {
        target
    }
}

fn part2(input: &[u64], target: u64) -> u64 {
    let mut imin = 0;
    let mut imax = 2;
    loop {
        let res: u64 = input[imin..imax].iter().sum();
        if res > target {
            imin += 1;
        } else if res < target {
            imax += 1;
        } else {
            let min = input[imin..imax].iter().min().unwrap();
            let max = input[imin..imax].iter().max().unwrap();
            break min + max;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];
        assert_eq!(part1(&input, 5), 127);
        assert_eq!(part2(&input, 127), 62);
        Ok(())
    }
}
