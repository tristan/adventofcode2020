use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn parse_input<R: BufRead>(reader: R) -> (usize, String) {
    let mut ingredients_count: HashMap<String, usize> = HashMap::new();
    let mut allergens_sets: HashMap<String, HashSet<String>> = HashMap::new();
    reader.lines().filter_map(Result::ok)
        .for_each(|line| {
            let idx = line.find("(contains ").unwrap();
            let idx2 = line.find(")").unwrap();
            let ingredients = line[..idx-1].split(" ")
                .map(str::to_owned)
                .collect::<HashSet<String>>();
            let allergens = line[idx+10..idx2].split(", ")
                .map(str::to_owned)
                .collect::<Vec<String>>();
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
    let combined = allergens_sets.iter().fold(HashSet::new(), |mut acc, (_, set)| {
        set.into_iter().for_each(|i| {
            acc.insert(i.clone());
        });
        acc
    });
    let ingredients: HashSet<String> = ingredients_count.keys().cloned().collect();
    (
        ingredients.difference(&combined).map(|k| {
            ingredients_count.get(k).unwrap()
        }).sum(),
        {
            let mut allergens = allergens_sets.keys().cloned().collect::<Vec<String>>();
            allergens.sort();
            let mut cdil: Vec<Option<String>> = vec![None; allergens.len()];
            while cdil.iter().any(Option::is_none) {
                let (i, ing) = allergens.iter().enumerate().find_map(|(i, allergen)| {
                    let set = allergens_sets.get(allergen).unwrap();
                    if set.len() == 1 {
                        Some((i, set.iter().cloned().next().unwrap()))
                    } else {
                        None
                    }
                }).unwrap();
                allergens.iter().for_each(|allergen| {
                    let s = allergens_sets.get_mut(allergen).unwrap();
                    s.remove(&ing);
                });
                cdil[i] = Some(ing);
            }
            cdil.into_iter().map(Option::unwrap).collect::<Vec<_>>().join(",")
        }
     )
}

fn main() {
    let f = File::open("day_21_input.txt").unwrap();
    let (part1, part2) = parse_input(BufReader::new(f));
    println!("part1: {}", part1);
    println!("part2: {}", part2);
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
        assert_eq!(
            parse_input(input.as_bytes()),
            (5, "mxmxvkd,sqjhc,fvjkl".to_string())
        );
    }
}
