use std::{collections::{BTreeMap, BTreeSet}, error::Error, fs::read_to_string};
use lazy_static::lazy_static;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
}

fn parse_mem(line: &str) -> (u64, u64) {
    let captures = MEM_RE.captures(line).expect("invalid line");
    let address = captures.name("address").expect("invalid line").as_str().parse().expect("invalid address");
    let value = captures.name("value").expect("invalid line").as_str().parse().expect("invalid value");

    (address, value)
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    
    let mut mask = String::new();
    let mut ones_mask: u64 = 0;
    let mut zeros_mask: u64 = 0;
    let mut memory: BTreeMap<u64, u64> = BTreeMap::new();
    let mut floating_memory: BTreeMap<u64, u64> = BTreeMap::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line.to_string().replace("mask = ", "");
            ones_mask = 0;
            zeros_mask = 0;

            for (nth_bit, c) in mask.chars().rev().enumerate() {
                match c {
                    '1' => ones_mask |= 1 << nth_bit,
                    '0' => zeros_mask |= 1 << nth_bit,
                    _ => (),
                };
            }
        } else {
            // Part 1
            let (address, value) = parse_mem(line);
            *memory.entry(address).or_default() = (value | ones_mask) & !zeros_mask;


            // Part 2
            let mut floating_addresses: BTreeSet<u64> = BTreeSet::new();
            let address_base = address | ones_mask;

            floating_addresses.insert(address_base);
            *floating_memory.entry(address_base).or_default() = value;


            for (nth_bit, c) in mask.chars().rev().enumerate() {
                if c != 'X' { continue; }
                let mut new_addresses: BTreeSet<u64> = BTreeSet::new();

                for addr in floating_addresses.iter() {
                    let (z, o) = (*addr & !(1 << nth_bit), *addr | 1 << nth_bit);

                    new_addresses.insert(z);
                    new_addresses.insert(o);

                    *floating_memory.entry(z).or_default() = value;
                    *floating_memory.entry(o).or_default() = value;
                }

                floating_addresses.append(&mut new_addresses);
            } 
        }
    }

    println!("Part 1: {}", memory.values().sum::<u64>());
    println!("Part 2: {}", floating_memory.values().sum::<u64>());

    Ok(())
}
