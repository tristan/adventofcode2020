use std::collections::{HashMap, HashSet};
use adventofcode2020::{
    ReadError,
    parse_input_file
};

#[derive(Clone)]
struct Rule {
    container: String,
    contains: Vec<(usize, String)>
}

struct Tree(HashMap<String, Vec<(usize, String)>>);

impl From<Vec<Rule>> for Tree {
    fn from(rules: Vec<Rule>) -> Tree {
        Tree(rules.into_iter()
             .map(|r| (r.container, r.contains))
             .collect())
    }
}

impl Tree {
    fn count_bags(&self, color: &str) -> usize {
        self.0.get(color)
            .map(|children| {
                children.iter().map(|(no, color)| {
                    no + (no * self.count_bags(color))
                }).sum()
            })
            .unwrap_or(0)
    }
}

fn parse_rule(line_no: usize, line: String) -> Result<Rule, ReadError> {
    let mut s = line.split(" bags contain ");
    let container = s.next()
        .ok_or_else(|| ReadError::ParseError(line_no, line.clone()))?
        .to_string();
    let contains = s.next()
        .map(|contains| {
            if contains == "no other bags." {
                Ok(vec![])
            } else {
                contains.trim_end_matches(".").split(", ")
                    .map(|bag: &str| -> Result<(usize, String), ReadError> {
                        let mut s = bag.split(" ");
                        let no: usize = s.next().map(|no| {
                            no.parse::<usize>()
                                .map_err(|_e| {
                                    ReadError::ParseError(line_no, line.clone())
                                })
                        }).ok_or_else(|| {
                            ReadError::ParseError(line_no, line.clone())
                        })??;
                        let bag = s.take(2).collect::<Vec<&str>>().join(" ");
                        Ok((no, bag))
                    })
                    .collect::<Result<Vec<(usize, String)>, ReadError>>()
            }
        })
        .unwrap_or_else(|| Err(ReadError::ParseError(line_no, line.clone())))?;
    Ok(Rule { container, contains })
}

fn build_tree(rules: Vec<Rule>) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in rules {
        for contains in rule.contains {
            if let Some(entry) = result.get_mut(&contains.1) {
                entry.insert(rule.container.clone());
            } else {
                let mut entry = HashSet::new();
                entry.insert(rule.container.clone());
                result.insert(contains.1, entry);
            }
        }
    }

    result
}

fn traverse_tree(color: &str, tree: &mut HashMap<String, HashSet<String>>) -> HashSet<String> {
    if let Some(parents) = tree.remove(color) {
        parents.into_iter().map(|color| {
            let mut r = traverse_tree(&color, tree);
            r.insert(color);
            r
        })
            .fold(HashSet::new(), |mut acc, set| {
                acc.extend(set.into_iter());
                acc
            })
    } else {
        HashSet::new()
    }

}

fn part1(mut tree: HashMap<String, HashSet<String>>) -> usize {
    let colors = traverse_tree("shiny gold", &mut tree);
    colors.len()
}

fn part2(tree: Tree) -> usize {
    tree.count_bags("shiny gold")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rules: Vec<Rule> = parse_input_file("day_07_input.txt", parse_rule)?;
    let tree = build_tree(rules.clone());
    let result = part1(tree);
    println!("part1: {}", result);
    let tree: Tree = rules.into();
    let result = part2(tree);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use adventofcode2020::parse_input_lines;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

        let rules: Vec<Rule> = parse_input_lines(input.as_bytes(), parse_rule)?;
        assert_eq!(rules.len(), 9);
        let tree = build_tree(rules.clone());
        assert_eq!(part1(tree), 4);

        let tree: Tree = rules.into();
        assert_eq!(part2(tree), 32);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;
        let tree: Tree = parse_input_lines(input.as_bytes(), parse_rule)?;
        assert_eq!(part2(tree), 126);
        Ok(())
    }
}
