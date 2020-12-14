use std::{fs, io::{BufRead, BufReader}};
use std::collections::HashMap;


fn part1<R>(reader: R) -> u64
where R: BufRead {
    let mut m1 = 0;
    let mut m2 = 0;
    let mut memory = HashMap::new();
    reader.lines().enumerate()
        .for_each(|(i, line)| {
            let line = line.unwrap_or_else(|_| panic!("Unable to read line: {}", i + 1));
            if line.starts_with("mask = ") {
                let (m1x, m2x) = line[7..].chars().fold((0, 0), |(m1, m2), c| {
                    let (v1, v2) = match c {
                        '1' => (1, 1),
                        '0' => (0, 0),
                        'X' => (1, 0),
                        _ => panic!("invalid value in mask: {}", c)
                    };
                    ((m1 << 1) + v1, (m2 << 1) + v2)
                });
                m1 = m1x;
                m2 = m2x;
            } else {
                let addr = line[4..line.find("]").unwrap()].parse::<usize>().unwrap();
                let val = line[line.find("=").unwrap() + 2..].parse::<u64>().unwrap();
                memory.insert(addr, (val & m1) | m2);
            }
        });
    memory.values().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open("day_14_input.txt")?;
    let result = part1(BufReader::new(file));
    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
"#;
        assert_eq!(part1(input.as_bytes()), 165);
        Ok(())
    }

}
