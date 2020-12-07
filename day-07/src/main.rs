use std::{collections::HashMap, error::Error, str::{Split, SplitN}, fs::read_to_string};

type InnerBag = HashMap<String, usize>;
type BagRuleset = HashMap<String, InnerBag>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let bag_ruleset: BagRuleset = input.lines().map(|line| {
        let mut part_iter = line.split(" contain ");
        let bag_name = normalize_bag_name(advance_line_part(&mut part_iter)?);
        let contents = advance_line_part(&mut part_iter)?;

        if contents.contains("no other bags") {
            return Ok((bag_name, InnerBag::new()));
        }
        
        let inner_bags: InnerBag = contents[..contents.len()-1].split(", ").map(|inner_desc| {
            let mut inner_desc_iter = inner_desc.splitn(2, " ");
            let count = advance_line_part(&mut inner_desc_iter)?.parse::<usize>()?;
            let name = normalize_bag_name(advance_line_part(&mut inner_desc_iter)?);
            Ok((name, count))
        }).collect::<Result<InnerBag>>()?;

        Ok((bag_name, inner_bags))
    }).collect::<Result<BagRuleset>>()?;

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

fn advance_line_part<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<&'a str>  {
    iter.next().ok_or("invalid line".into())
}
