use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn parse_input<R: BufRead>(reader: R) -> usize {
    let mut ingredients_count: HashMap<String, usize> = HashMap::new();
    let mut allergens_sets: HashMap<String, HashSet<String>> = HashMap::new();
    reader.lines().filter_map(Result::ok)
        .for_each(|line| {
            let idx = line.find("(contains ").unwrap();
            let idx2 = line.find(")").unwrap();
            let ingredients = line[..idx].split(" ").map(str::to_owned).collect::<HashSet<String>>();
            let allergens = line[idx+10..idx2].split(", ").map(str::to_owned).collect::<Vec<String>>();
            allergens.into_iter().for_each(|allergen| {
                if let Some(set) = allergens_sets.get(&allergen) {
                    let int = set.intersection(&ingredients).cloned().collect();
                    allergens_sets.insert(allergen, int);
                } else {
                    allergens_sets.insert(allergen, ingredients.clone());
                }
            });
            ingredients.into_iter().for_each(|ingredient| {
                let val = ingredients_count.remove(&ingredient).unwrap_or(0) + 1;
                ingredients_count.insert(ingredient, val);
            });
        });
    // find all ingredients that are not in any allergen sets
    // combine allergen sets
    let combined = allergens_sets.into_iter().fold(HashSet::new(), |mut acc, (_, set)| {
        set.into_iter().for_each(|i| {
            acc.insert(i);
        });
        acc
    });
    let ingredients: HashSet<String> = ingredients_count.keys().cloned().collect();
    ingredients.difference(&combined).map(|k| {
        ingredients_count.get(k).unwrap()
    }).sum()
}

fn main() {
    let f = File::open("day_21_input.txt").unwrap();
    let result = parse_input(BufReader::new(f));
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"#;
        assert_eq!(parse_input(input.as_bytes()), 5);
    }
}
