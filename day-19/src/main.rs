#![feature(str_split_once)]

use std::{collections::HashMap, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type RuleRef = usize;
type RuleSet = HashMap<RuleRef, RuleEntry>;

#[derive(Debug)]
struct RuleEntry {
    id: RuleRef,
    rule: RuleDef,
}

impl RuleEntry {
    fn parse_id(id: &str) -> RuleRef {
        id.parse::<RuleRef>().expect("invalid rule reference")
    }

    fn parse_references(references: &str) -> Vec<RuleRef> {
        references.split_whitespace().map(|id| Self::parse_id(id)).collect()
    }
}

#[derive(Debug)]
enum RuleDef {
    Simple(char),
    Compound(Vec<RuleRef>),
    Either{ left: Vec<RuleRef>, right: Vec<RuleRef> },
}


fn resolve(input: &str, reference: &RuleRef, ruleset: &RuleSet) -> (bool, usize) {
    fn resolve_compound(input: &str, references: &Vec<RuleRef>, ruleset: &RuleSet) -> (bool, usize) {
        references.iter().fold((true, 0), |(matches, covered), r| {
            let (is_match, offset) = resolve(&input[covered..], r, &ruleset);
            (matches && is_match, covered + offset)
        })
    }

    match &ruleset.get(reference).expect("no reference entry found").rule {
        RuleDef::Simple(c) => {
            let matches = input.chars().next().unwrap() == *c;
            (matches, 1)
        },
        RuleDef::Compound(references) => resolve_compound(input, references, &ruleset),
        RuleDef::Either { left, right } => {
            let (left_matches, left_covered) = resolve_compound(input, left, &ruleset);
            let (right_matches, right_covered) = resolve_compound(input, right, &ruleset);
            assert!(left_covered == right_covered);
            (left_matches || right_matches, left_covered)
        },
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let (rules, messages) = input.split_once("\n\n").ok_or("invalid input")?;

    let ruleset: RuleSet = rules.lines().map(|line| {
        match line.split_once(": ") {
            Some((id, part)) if part.contains("\"") => {
                RuleEntry{ 
                    id: RuleEntry::parse_id(id),
                    rule: RuleDef::Simple(part.chars().nth(1).expect("invalid rule"))
                }
            },
            Some((id, part)) if part.contains(" | ") => {
                let (left_refs, right_refs) = part.split_once(" | ").expect("invalid rule");
                
                RuleEntry{
                    id: RuleEntry::parse_id(id),
                    rule: RuleDef::Either{ 
                        left: RuleEntry::parse_references(left_refs),
                        right: RuleEntry::parse_references(right_refs),
                    }
                }
            },
            Some((id, part)) => {
                RuleEntry{
                    id: RuleEntry::parse_id(id),
                    rule: RuleDef::Compound(RuleEntry::parse_references(part))
                }
            },
            None => panic!(format!("invalid rule: {}", line)),
        }
    }).map(|entry| (entry.id, entry)).collect();

    let num_valid = messages.lines().filter(|line| {
        match resolve(line, &0, &ruleset) {
            (true, covered) => covered == line.len(),
            (false, _) => false,
        }
    }).count();

    println!("Part 1: {}", num_valid);


    let num_valid = messages.lines().filter(|line| {
        // 0: 8 11
        // 8: 42 | 42 8 => 42 | 42 42 | 42 42 42 | ...
        // 11: 42 31 | 42 11 31 => 42 31 | 42 42 31 31 | 42 42 42 31 31 31 | ...
        // 0: 42 42 31 | 42 42 42 31 | 42 42 42 42 31 | 42 42 42 31 31 | ...
        let (mut num_42_valid, mut num_31_valid) = (1, 0);
        let (first_valid, mut offset) = resolve(line, &42, &ruleset);

        if !first_valid { return false; }

        while offset < line.len() {
            match resolve(&line[offset..], &42, &ruleset) {
                (true, covered) => { offset += covered; num_42_valid += 1; },
                (false, _) => { break; },
            }
        }

        if num_42_valid == 0 || offset == line.len() { return false; }

        while offset < line.len() {
            match resolve(&line[offset..], &31, &ruleset) {
                (true, covered) => { offset += covered; num_31_valid += 1; },
                (false, _) => break,
            }
        }

        num_42_valid > num_31_valid && offset == line.len()
    }).count();

    println!("Part 2: {}", num_valid);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        let mut ruleset = RuleSet::new();

        ruleset.insert(0, RuleEntry{id: 0, rule: RuleDef::Compound(vec![1, 2])});
        ruleset.insert(1, RuleEntry{id: 1, rule: RuleDef::Simple('a')});
        ruleset.insert(2, RuleEntry{id: 2, rule: RuleDef::Either{left: vec![1, 3], right: vec![3, 1]}});
        ruleset.insert(3, RuleEntry{id: 3, rule: RuleDef::Simple('b')});
        
        assert!(resolve(&mut "aab".to_string(), &0, &ruleset).0);
        assert!(resolve(&mut "aba".to_string(), &0, &ruleset).0);
    }

    #[test]
    fn test_part_1_sample_2() {
        let mut ruleset = RuleSet::new();

        ruleset.insert(0, RuleEntry{id: 0, rule: RuleDef::Compound(vec![4, 1, 5])});
        ruleset.insert(1, RuleEntry{id: 1, rule: RuleDef::Either{left: vec![2, 3], right: vec![3, 2]}});
        ruleset.insert(2, RuleEntry{id: 2, rule: RuleDef::Either{left: vec![4, 4], right: vec![5, 5]}});
        ruleset.insert(3, RuleEntry{id: 3, rule: RuleDef::Either{left: vec![4, 5], right: vec![5, 4]}});
        ruleset.insert(4, RuleEntry{id: 4, rule: RuleDef::Simple('a')});
        ruleset.insert(5, RuleEntry{id: 5, rule: RuleDef::Simple('b')});

        assert!(resolve("aa", &2, &ruleset).0);
        assert!(resolve("bb", &2, &ruleset).0);
        assert!(resolve("ab", &3, &ruleset).0);
        assert!(resolve("ba", &3, &ruleset).0);
        assert!(resolve("aaab", &1, &ruleset).0);
        
        assert!(resolve("ababbb", &0, &ruleset).0);
        assert!(resolve("abbbab", &0, &ruleset).0);
        assert!(!resolve("bababa", &0, &ruleset).0);

        assert_eq!(resolve("aaaabbb", &0, &ruleset), (true, 6));
    }
}
