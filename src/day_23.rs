fn parse_input(input: &str) -> Vec<u8> {
    input.chars().map(|c| c as u8 - '0' as u8).collect()
}

fn part1(mut cups: Vec<u8>) -> String {
    let largest: u8 = *cups.iter().max().unwrap();
    for _ in 0..100 {
        let mut picked: Vec<u8> = vec![
            cups.remove(1),
            cups.remove(1),
            cups.remove(1)
        ];
        let mut dest = cups[0] - 1;
        if dest == 0 {
            dest = largest;
        }
        while picked.contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = largest;
            }
        }
        let dest_idx = cups.iter().position(|&cup| cup == dest).unwrap();
        cups.insert(dest_idx + 1, picked.pop().unwrap());
        cups.insert(dest_idx + 1, picked.pop().unwrap());
        cups.insert(dest_idx + 1, picked.pop().unwrap());

        cups.rotate_left(1)
    }

    let one_idx = cups.iter().position(|&cup| cup == 1).unwrap();
    if one_idx > 0 {
        cups.rotate_left(one_idx);
    }
    cups.into_iter().skip(1).map(|cup| (cup + '0' as u8) as char).collect()
}

fn part2(input: Vec<u8>) -> u64 {
    let largest: u32 = 1000000;
    let mut current: u32 = input[0] as u32;
    let mut cups: Vec<u32> = vec![0; largest as usize + 1];
    input.windows(2).for_each(|arr| cups[arr[0] as usize] = arr[1] as u32);
    cups[input[input.len() - 1] as usize] = 10;
    cups[largest as usize] = current;
    (10..largest).for_each(|i| cups[i as usize] = i + 1);
    for _ in 0..10000000 {
        let p1 = cups[current as usize];
        let p2 = cups[p1 as usize];
        let p3 = cups[p2 as usize];
        let cn = cups[p3 as usize];

        let mut dest = current - 1;
        while dest == 0 || dest == p1 || dest == p2 || dest == p3 {
            dest = if dest == 0 {
                largest
            } else {
                dest - 1
            }
        }
        let dn = cups[dest as usize];

        cups[current as usize] = cn;
        cups[dest as usize] = p1;
        cups[p3 as usize] = dn;

        current = cn;
    }

    let a1 = cups[1];
    let a2 = cups[a1 as usize];
    (a1 as u64) * (a2 as u64)
}

fn main() {
    let input = "215694783";
    let cups = parse_input(&input);
    let result = part1(cups.clone());
    println!("part1: {}", result);
    let s = std::time::Instant::now();
    let result = part2(cups);
    println!("part2: {} ({:?})", result, s.elapsed());

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let data = parse_input("0123456789");
        assert_eq!(data, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_part1() {
        let data = parse_input("389125467");
        assert_eq!(part1(data), "67384529");
    }

    #[test]
    fn test_part2() {
        let data = parse_input("389125467");
        assert_eq!(part2(data), 149245887792);
    }


}
