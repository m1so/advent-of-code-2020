#![feature(str_split_once)]

use std::{collections::{HashMap, HashSet}, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let mut allergen_candidates: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients_list: Vec<HashSet<String>> = Vec::new();

    for line in input.lines() {
        let (food_list, allergens) = line.split_once(" (contains ").ok_or("invalid list")?;
        let food_list: HashSet<String> = food_list.split_whitespace().map(|s| s.to_string()).collect();
        let allergens: Vec<&str> = allergens[..allergens.len()-1].split(", ").collect();

        ingredients_list.push(food_list.clone());

        for allergen in allergens {
            let old_list = allergen_candidates.entry(allergen.to_string()).or_insert(food_list.clone());
            let new_list: HashSet<String> = old_list.intersection(&food_list).cloned().collect();
            *old_list = new_list;
        }
    }

    let all_allergens = allergen_candidates
        .values()
        .fold(HashSet::new(), |mut all, allergens| {
            all.extend(allergens.iter().cloned());
            all
        });

    let non_allergens_count = ingredients_list
        .iter()
        .map(|ingredients| {
            ingredients.difference(&all_allergens).collect::<Vec<_>>().len()
        })
        .sum::<usize>();
    
    println!("Part 1: {}", non_allergens_count);
    
    let mut candidates: Vec<(String, HashSet<String>)> = allergen_candidates
        .iter()
        .map(|(k, v)| (k.clone(), v.clone())).collect();
    
    loop {
        for i in 0..candidates.len() {
            if candidates[i].1.len() != 1 { continue; }
            
            for j in 0..candidates.len() {
                if i == j { continue; }
                
                let diff: HashSet<String> = candidates[j].1.difference(&candidates[i].1).cloned().collect();
                candidates[j].1 = diff;
            }
        }

        if candidates.iter().all(|(_, allergens)| allergens.len() == 1) {
            break;
        }
    }

    candidates.sort_by(|(a, _), (b, _)| a.cmp(b));

    let ingredients: Vec<String> = candidates
        .iter()
        .filter(|(_, ingredients)| ingredients.len() > 0)
        .map(|(_, ingredients)|  ingredients.iter().cloned().collect::<Vec<String>>().join(","))
        .collect();

    println!("Part 2: {}", ingredients.join(","));

    Ok(())
}
