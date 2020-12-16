use std::{fs, io::{BufRead, BufReader}};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rule(usize, usize, usize, usize, bool);

impl Rule {
    fn matches(&self, val: usize) -> bool {
        (self.0 <= val && self.1 >= val) || (self.2 <= val && self.3 >= val)
    }
}

struct Ticket(Vec<usize>);

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.0.iter()
            .all(|&v| rules.iter().any(|r| r.matches(v)))
    }
}

fn parse_input<R>(reader: R) -> (Vec<Rule>, Vec<Ticket>)
where R: BufRead {
    let mut rules = vec![];
    let mut tickets = vec![];
    reader.lines()
        .filter_map(Result::ok)
        .filter(|line| line.len() > 0 && line != "your ticket:" && line != "nearby tickets:")
        .for_each(|line| {
            if line.chars().next().unwrap().is_numeric() {
                let ticket = line.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect();
                tickets.push(Ticket(ticket));
            } else {
                let departure = line.starts_with("departure");
                let idx1 = line.find(":").unwrap() + 2;
                let idx2 = line[idx1..].find("-").unwrap() + idx1;
                let idx3 = line[idx2..].find(" or ").unwrap() + idx2;
                let idx4 = idx3 + 4;
                let idx5 = line[idx4..].find("-").unwrap() + idx4;
                let v1 = line[idx1..idx2].parse().unwrap();
                let v2 = line[idx2+1..idx3].parse().unwrap();
                let v3 = line[idx4..idx5].parse().unwrap();
                let v4 = line[idx5+1..].parse().unwrap();
                rules.push(Rule(v1, v2, v3, v4, departure));
            }
        });
    (rules, tickets)
}

fn get_ordered_rules(rules: &[Rule], tickets: &[Ticket]) -> Vec<Rule> {
    let tickets = tickets.iter()
        .filter(|t| t.is_valid(rules))
        .collect::<Vec<&Ticket>>();

    let mut ordered_rules: Vec<Option<Rule>> = vec![None; rules.len()];
    while ordered_rules.iter().any(Option::is_none) {
        (0..rules.len())
            .for_each(|i| {
                if ordered_rules[i].is_none() {
                    let rules = rules.iter()
                        .filter(|&r1| {
                            !ordered_rules.contains(&Some(*r1))
                        })
                        .cloned().collect::<Vec<Rule>>();
                    let remaining_rules = tickets.iter().map(|t| t.0[i])
                        .fold(rules, |rules, val| {
                            let rules = rules.into_iter()
                                .filter(|r| {
                                    let m = r.matches(val);
                                    m
                                })
                                .collect();
                            rules
                        });
                    if remaining_rules.len() == 1 {
                        let rule = remaining_rules.into_iter().next().unwrap();
                        ordered_rules[i] = Some(rule);
                    } else if remaining_rules.len() == 0 {
                        panic!("got no remaining rules");
                    }
                }
            });
    }
    ordered_rules.into_iter().map(Option::unwrap).collect()
}

fn part1(rules: &[Rule], tickets: &[Ticket]) -> usize {
    tickets.iter().filter_map(|t| {
        t.0.iter().find(|&&v| {
            rules.iter().all(|r| !r.matches(v))
        })
    }).sum()
}

fn part2(rules: &[Rule], tickets: &[Ticket]) -> usize {
    let rules = get_ordered_rules(rules, tickets);
    let ticket = &tickets[0];
    rules.iter().zip(ticket.0.iter())
        .filter(|(r, _)| r.4)
        .map(|(_, c)| c)
        .product()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = fs::File::open("day_16_input.txt")?;
    let r = BufReader::new(f);
    let (rules, tickets) = parse_input(r);
    let result = part1(&rules, &tickets);
    println!("part1: {}", result);
    let result = part2(&rules, &tickets);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2() {
        let input = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"#;
        let (rules, tickets) = parse_input(input.as_bytes());
        dbg!(&rules);
        let ordered_rules = get_ordered_rules(&rules, &tickets[1..]);
        dbg!(ordered_rules);
    }
}
