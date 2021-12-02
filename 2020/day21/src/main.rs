use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::{HashSet, HashMap, BTreeMap};

type Input = Lines<BufReader<File>>;

fn get_file_lines(file_name: &str) -> Input {
    let path = Path::new(file_name);
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn parse_ingredients(line: String) -> (HashSet<String>, HashSet<String>) {
    let pieces = line.split(" (contains ").collect::<Vec<&str>>();
    let ingredients = pieces[0].split(" ")
        .map(|ingredient| ingredient.to_owned())
        .collect::<HashSet<String>>();
    let allergens = match pieces.len() == 2 {
        true => pieces[1]
            .strip_suffix(")").unwrap()
            .split(", ")
            .map(|allergen| allergen.to_owned())
            .collect::<HashSet<String>>(),
        false => HashSet::new()
    };
    (ingredients, allergens)
}

fn test_input(file_name: &str) {
    let lines = get_file_lines(file_name);

    let mut candidates = HashMap::<String, HashSet<String>>::new();

    let ingredients_and_allergens = lines.flat_map(|line| line.ok())
        .map(|line| parse_ingredients(line));
    
    let mut ingredient_counts = HashMap::<String, usize>::new();
    
    for (ingredients, allergens) in ingredients_and_allergens {
        for allergen in allergens {
            if let Some(existing) = candidates.get_mut(&allergen) {
                existing.retain(|ingredient| ingredients.contains(ingredient));
            } else {
                candidates.insert(allergen, ingredients.clone());
            }
        }
        for ingredient in ingredients {
            let count = ingredient_counts.entry(ingredient).or_default();
            (*count) = *count + 1;
        }
    }

    // keep removing from the candidates when we have
    // one ingredient for a allergen->ingredients mapping
    // until no changes have been made.
    let mut changed = true;
    let mut ingredient_to_allergen = HashMap::new();
    while changed {
        changed = false;
        candidates.retain(|allergen, ingredients| {
            if ingredients.len() != 1 {
                return true;
            }

            let ingredient = ingredients.iter().last().unwrap().clone();
            ingredient_to_allergen.insert(ingredient, allergen.clone());
            changed = true;
            false
        });
        candidates.values_mut().for_each(|ingredients| {
            ingredients.retain(|ingredient| {
                let remove = ingredient_to_allergen.contains_key(ingredient);
                changed = changed | remove;
                !remove
            });
        });
    }

    let safe_ingredient_counts = ingredient_counts.iter()
        .filter(|ingredient| !ingredient_to_allergen.contains_key(ingredient.0))
        .map(|ingredient| (ingredient.0.clone(), *ingredient.1))
        .collect::<HashMap<String, usize>>();
    
    let safe_occurrences = safe_ingredient_counts.values().sum::<usize>();
    println!("For {}, there are {} safe ingredients with {} occurrences", file_name, safe_ingredient_counts.len(), safe_occurrences);

    let canonical_dangerous_ingredients = ingredient_to_allergen.iter()
        .map(|entry| (entry.1.clone(), entry.0.clone()))
        .collect::<BTreeMap<String, String>>()
        .iter()
        .map(|entry| entry.1.clone())
        .collect::<Vec<String>>()
        .join(",");
    
    println!("For {}, the canonical dangerous ingredient list is: {}", file_name, canonical_dangerous_ingredients);
}

fn main() {
    test_input("sample.txt");
    test_input("input.txt");
}
