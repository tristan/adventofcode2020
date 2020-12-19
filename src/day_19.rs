use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Rule {
    Char(u8),
    Rule(usize),
}

struct RuleSet(HashMap<usize, Vec<Vec<Rule>>>);

fn parse_input<R: BufRead>(reader: R) -> (RuleSet, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();
    let mut mode = 0;
    reader.lines()
        .filter_map(Result::ok)
        .for_each(|line| {
            if line.len() == 0 {
                mode = 1;
            } else if mode == 0 {
                // parse rule
                let (idx, chains) = match line.splitn(2, ": ").collect::<Vec<_>>().as_slice() {
                    &[idx, chains] => (
                        idx.parse::<usize>().expect("invalid input"),
                        chains.split(" | ").map(|chain| {
                            chain.split(" ").map(|v| match v.parse::<usize>() {
                                Ok(i) => Rule::Rule(i),
                                Err(_) => Rule::Char(v.as_bytes()[1])
                            }).collect::<Vec<_>>()
                        }).collect::<Vec<_>>()
                    ),
                    r => {
                        dbg!(r);
                        panic!("invalid input")
                    }
                };
                rules.insert(idx, chains);
            } else {
                messages.push(line);
            }
        });
    (RuleSet(rules), messages)
}

impl RuleSet {
    fn check_rule(&self, num: usize, message: &str) -> Option<usize> {
        self.0.get(&num)
            .map(|possibilities| {
                possibilities.iter().find_map(|chain| {
                    let mut idx = 0;
                    let matched = chain.iter().all(|rule| {
                        if idx >= message.len() {
                            return false;
                        }
                        match rule {
                            Rule::Char(c) => {
                                if message.as_bytes()[idx] == *c {
                                    idx += 1;
                                    true
                                } else {
                                    false
                                }
                            },
                            Rule::Rule(r) => {
                                if let Some(offset) = self.check_rule(*r, &message[idx..]) {
                                    idx += offset;
                                    true
                                } else {
                                    false
                                }
                            }
                        }
                    });
                    if matched {
                        Some(idx)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(None)
    }

    fn validate_part1(&self, message: &str) -> bool {
        if let Some(offset) = self.check_rule(0, message) {
            message.len() == offset
        } else {
            false
        }
    }

    fn validate_part2(&mut self, message: &str) -> bool {
        for eights in 1..20 {
            for elevens in 1..20 {
                self.prepare_part2_iteration(eights, elevens);
                let matched = self.validate_part1(message);
                if matched {
                    return true;
                }
            }
        }
        false
    }

    fn prepare_part2_iteration(&mut self, eights: usize, elevens: usize) {
        let eight = vec![vec![Rule::Rule(42); eights]];
        self.0.insert(8, eight);
        let mut eleven = vec![];
        for _ in 0..elevens {
            eleven.push(Rule::Rule(42));
        }
        for _ in 0..elevens {
            eleven.push(Rule::Rule(31));
        }
        self.0.insert(11, vec![eleven]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open("day_19_input.txt")?;
    let (mut ruleset, messages) = parse_input(BufReader::new(f));
    let part1 = messages.iter().filter(|msg| ruleset.validate_part1(msg))
        .count();
    println!("part1: {}", part1);
    let part2 = messages.iter().filter(|msg| ruleset.validate_part2(msg))
        .count();
    println!("part2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
        let (ruleset, messages) = parse_input(input.as_bytes());
        assert!(ruleset.validate_part1(&messages[0]));
        assert!(ruleset.validate_part1(&messages[2]));
        assert!(!ruleset.validate_part1(&messages[1]));
        assert!(!ruleset.validate_part1(&messages[3]));
        assert!(!ruleset.validate_part1(&messages[4]));
    }

    #[test]
    fn test_2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

        let (mut ruleset, messages) = parse_input(input.as_bytes());
        let part1 = messages.iter().filter(|msg| ruleset.validate_part1(msg))
            .count();
        assert_eq!(part1, 3);
        assert!(!ruleset.validate_part1("bbbababbbbaaaaaaaabbababaaababaabab"));
        assert!(!ruleset.validate_part1("aaaaabbaabaaaaababaa"));
        assert!(ruleset.validate_part2("bbbababbbbaaaaaaaabbababaaababaabab"));
        assert!(ruleset.validate_part2("aaaaabbaabaaaaababaa"));
        let part2 = messages.iter().filter(|msg| ruleset.validate_part2(msg))
            .count();
        assert_eq!(part2, 12);
    }

}
