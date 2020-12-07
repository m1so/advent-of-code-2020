use std::{error::Error, collections::HashMap, fs::read_to_string};

type InnerBag = HashMap<String, usize>;
type BagRuleset = HashMap<String, InnerBag>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/input1.txt")?;

    let bag_ruleset: BagRuleset = input.lines().map(|line| {
        let mut part_iter = line.split(" contain ");
        let bag_name = normalize_bag_name(part_iter.next().unwrap());
        let contents = part_iter.next().unwrap();

        if contents.contains("no other bags") {
            return (bag_name, InnerBag::new());
        }
        
        let inner_bags: InnerBag = contents[..contents.len()-1].split(", ").map(|inner_desc| {
            let mut inner_desc_iter = inner_desc.splitn(2, " ");
            let count = inner_desc_iter.next().unwrap().parse::<usize>().unwrap();
            let name = normalize_bag_name(inner_desc_iter.next().unwrap());
            (name, count)
        }).collect();

        (bag_name, inner_bags)
    }).collect();

    let num_shiny_gold_bag_holders = bag_ruleset.values().filter(|inner_bag| contains_bag(&bag_ruleset, inner_bag, "shiny gold")).count();
    println!("Part 1: {}", num_shiny_gold_bag_holders);

    let bags_within_shiny_gold_bag = bag_count(&bag_ruleset, bag_ruleset.get("shiny gold").ok_or("shiny gold bag missing")?);
    println!("Part 2: {}", bags_within_shiny_gold_bag);

    Ok(())
}

fn contains_bag(bag_ruleset: &BagRuleset, inner_bag: &InnerBag, target: &str) -> bool {
    inner_bag.contains_key(target) || 
        inner_bag.keys().any(|inner_name| {
            bag_ruleset.contains_key(inner_name) 
                && contains_bag(bag_ruleset, bag_ruleset.get(inner_name).unwrap(), target)
        })
}

fn bag_count(bag_ruleset: &BagRuleset, inner_bag: &InnerBag) -> usize {
    inner_bag.iter().map(|(name, count)| count + count * bag_count(bag_ruleset, bag_ruleset.get(name).unwrap())).sum()
}

fn normalize_bag_name(bag_name: &str) -> String {
    bag_name.replace("bags", "").replace("bag", "").trim().to_string()
}
