use std::collections::HashMap;

fn solve(starting: &[usize], limit: usize) -> usize {
    let map = starting.iter().take(starting.len() - 1).enumerate()
        .map(|(i, c)| (*c, i + 1))
        .collect::<HashMap<usize, usize>>();
    let (_, last) = (starting.len()..limit)
        .fold((map, starting[starting.len() - 1]), |(mut map, last), turn| {
            let next = if let Some(prev) = map.get(&last) {
                turn - prev
            } else {
                0
            };
            map.insert(last, turn);
            (map, next)
        });
    last
}

fn main() {
    let input = [2, 1, 10, 11, 0, 6];
    let result = solve(&input, 2020);
    println!("part1: {}", result);
    let n = std::time::Instant::now();
    let result = solve(&input, 30000000);
    println!("part2: {} ({:?})", result, n.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1(&[0,3,6], 2020), 436);
        assert_eq!(part1(&[1,3,2], 2020), 1);
        assert_eq!(part1(&[2,1,3], 2020), 10);
        assert_eq!(part1(&[1,2,3], 2020), 27);
        assert_eq!(part1(&[2,3,1], 2020), 78);
        assert_eq!(part1(&[3,2,1], 2020), 438);
        assert_eq!(part1(&[3,1,2], 2020), 1836);
    }
}
