
const DIVISOR: usize = 20201227;

fn loop_step(val: usize, subject_number: usize) -> usize {
    let res = val * subject_number;
    res % DIVISOR
}

fn find_loop_size(subject_number: usize, public_key: usize) -> usize {
    let mut val = 1;
    let mut i = 0;
    loop {
        i += 1;
        val = loop_step(val, subject_number);
        if val == public_key {
            return i;
        }
    }
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    (0..loop_size).fold(1, |acc, _| loop_step(acc, subject_number))
}

fn part1(card_public_key: usize, door_public_key: usize) -> usize {
    let card_loop_size = find_loop_size(7, card_public_key);
    let door_loop_size = find_loop_size(7, door_public_key);
    let encryption_key_1 = transform(card_public_key, door_loop_size);
    let encryption_key_2 = transform(door_public_key, card_loop_size);
    assert_eq!(encryption_key_1, encryption_key_2);
    encryption_key_1
}

fn main() {
    let card_public_key = 18499292;
    let door_public_key = 8790390;
    let result = part1(card_public_key, door_public_key);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let card_public_key = 5764801;
        let door_public_key = 17807724;
        let subject_number = 7;

        let card_loop_size = find_loop_size(subject_number, card_public_key);
        assert_eq!(card_loop_size, 8);
        let door_loop_size = find_loop_size(subject_number, door_public_key);
        assert_eq!(door_loop_size, 11);

        let encryption_key_1 = transform(card_public_key, door_loop_size);
        assert_eq!(encryption_key_1, 14897079);
        let encryption_key_2 = transform(door_public_key, card_loop_size);
        assert_eq!(encryption_key_2, 14897079);
    }

    #[test]
    fn test_part1() {
        let card_public_key = 5764801;
        let door_public_key = 17807724;
        assert_eq!(part1(card_public_key, door_public_key), 14897079);
    }

}
