use std::{fs, io::{BufRead, BufReader}};

use adventofcode2020::ReadError;

fn parse<R>(reader: R) -> Result<(usize, Vec<Option<usize>>), ReadError>
where R: BufRead {
    let mut lines = reader.lines();
    let time = lines.next()
        .ok_or_else(|| ReadError::ParseError(1, "Unable to read line".to_string()))?
        .map_err(|_| ReadError::ParseError(1, "Unable to read line".to_string()))?;

    let time = time.parse()
        .map_err(|_| ReadError::ParseError(1, time))?;

    let busses = lines.next()
        .ok_or_else(|| ReadError::ParseError(1, "Unable to read line".to_string()))?
        .map_err(|_| ReadError::ParseError(1, "Unable to read line".to_string()))?;

    let busses = busses.split(",").map(|id| id.parse().ok()).collect();

    Ok((time, busses))
}

fn part1(time: usize, busses: &[Option<usize>]) -> usize {
    busses
        .iter()
        .filter_map(|b| *b)
        .fold(None, |cur, bus| {
            let wait = ((time + bus) - (time % bus)) - time;
            if let Some((_, cur_wait)) = cur {
                if cur_wait > wait {
                    Some((bus, wait))
                } else {
                    cur
                }
            } else {
                Some((bus, wait))
            }
        })
        .map(|(bus, wait)| bus * wait)
        .unwrap_or(0)
}

fn part2(busses: &[Option<usize>]) -> usize {
    busses.iter().enumerate()
        .filter_map(|(i, bus)| bus.map(|b| (i, b)))
        .fold(None, |prev, (i, b)| {
            if let Some((pi, pb)) = prev {
                let (step, step_offset, check, check_offset) = if pb > b { (pb, pi, b, i) } else { (b, i, pb, pi) };
                let (m, t1) = (1..)
                    .find_map(|m| {
                        let target = (m * step) - step_offset;
                        if (target + check_offset) % check == 0 {
                            Some((m, target))
                        } else {
                            None
                        }
                    })
                    .unwrap();
                let t2 = (m + 1..)
                    .find_map(|m| {
                        let target = (m * step) - step_offset;
                        if (target + check_offset) % check == 0 {
                            Some(target - t1)
                        } else {
                            None
                        }
                    })
                    .unwrap();
                Some((t2 - t1, t2))
            } else {
                Some((i, b))
            }
        })
        .map(|(o, t)| t - o)
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open("day_13_input.txt")?;
    let (time, busses) = parse(BufReader::new(f))?;
    let result = part1(time, &busses);
    println!("part1: {}", result);
    let result = part2(&busses);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = "939\n7,13,x,x,59,x,31,19";
        let (time, busses) = parse(input.as_bytes())?;
        assert_eq!(part1(time, &busses), 295);
        assert_eq!(part2(&busses), 1068781);
        let input = "0\n17,x,13,19";
        let (_, busses) = parse(input.as_bytes())?;
        assert_eq!(part2(&busses), 3417);
        let input = "0\n67,7,59,61";
        let (_, busses) = parse(input.as_bytes())?;
        assert_eq!(part2(&busses), 754018);
        let input = "0\n67,x,7,59,61";
        let (_, busses) = parse(input.as_bytes())?;
        assert_eq!(part2(&busses), 779210);
        let input = "0\n67,7,x,59,61";
        let (_, busses) = parse(input.as_bytes())?;
        assert_eq!(part2(&busses), 1261476);
        let input = "0\n1789,37,47,1889";
        let (_, busses) = parse(input.as_bytes())?;
        assert_eq!(part2(&busses), 1202161486);

        Ok(())
    }
}
