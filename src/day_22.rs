use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque, HashSet};

fn parse_input<R: BufRead>(reader: R) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut lines = reader.lines().filter_map(Result::ok).skip(1);
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        player1.push_back(line.parse::<usize>().unwrap());
    }
    lines.next().unwrap();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        player2.push_back(line.parse::<usize>().unwrap());
    }
    (player1, player2)
}

fn part1(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    loop {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
        return if player1.len() == 0 {
            &player2
        } else if player2.len() == 0 {
            &player1
        } else {
            continue
        }.iter().rev().enumerate().map(|(i, card)| {
            (i + 1) * card
        }).sum();
    }
}

fn recursive_combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>, game: usize) -> (usize, VecDeque<usize>) {
    let mut previous_rounds = HashSet::new();
    loop {
        if !previous_rounds.insert((deck1.clone(), deck2.clone())) {
            return (1, deck1);
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let winner = if deck1.len() >= card1 && deck2.len() >= card2 {
            let (winner, _) = recursive_combat(
                deck1.iter().take(card1).cloned().collect(),
                deck2.iter().take(card2).cloned().collect(),
                game + 1
            );
            winner
        } else {
            if card1 > card2 {
                1
            } else {
                2
            }
        };
        if winner == 1 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
        return if deck1.is_empty() {
            (2, deck2)
        } else if deck2.is_empty() {
            (1, deck1)
        } else {
            continue
        }
    }
}

fn part2(player1: VecDeque<usize>, player2: VecDeque<usize>) -> usize {
    let (_, winning_deck) = recursive_combat(player1.clone(), player2.clone(), 1);
    winning_deck.iter().rev().enumerate().map(|(i, card)| {
        (i + 1) * card
    }).sum()
}

fn main() {
    let f = File::open("day_22_input.txt").unwrap();
    let (player1, player2) = parse_input(BufReader::new(f));
    let result = part1(player1.clone(), player2.clone());
    println!("part1: {}", result);
    let s = std::time::Instant::now();
    let result = part2(player1, player2);
    println!("part2: {} ({:?})", result, s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"#;
        let (player1, player2) = parse_input(input.as_bytes());
        assert_eq!(part1(player1.clone(), player2.clone()), 306);
        assert_eq!(part2(player1.clone(), player2.clone()), 291);
    }

    #[test]
    fn test_infinite() {
        let input = r#"Player 1:
43
19

Player 2:
2
29
14
"#;
        let (player1, player2) = parse_input(input.as_bytes());
        part2(player1.clone(), player2.clone());
    }
}
