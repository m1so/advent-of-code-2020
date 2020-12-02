#![feature(bool_to_option)]

use std::{error::Error, io::{Read, stdin}, result::Result};

fn parse_line(line: &str) -> Option<(usize, usize, char, &str)> {
    let parts: Vec<&str> = line.split_ascii_whitespace().collect();
    assert_eq!(parts.len(), 3);

    let min_max: Vec<usize> = parts[0].split('-').map(|num| num.parse().unwrap()).take(2).collect();
    assert_eq!(min_max.len(), 2);

    let (min, max) = match &min_max[..] {
        &[min, max, ..] => (min, max),
        _ => unreachable!(),
    };

    let letter = parts[1].chars().nth(0)?;
    let password = parts[2];

    Some((min, max, letter, password))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    
    let part_1_result = input.lines().filter_map(|line| {
        let (min, max, letter, password) = parse_line(line)?;
        let letter_count = password.chars().filter(|char| *char == letter).count();

        (letter_count >= min && letter_count <= max).then_some(())
    }).count();

    println!("Part 1: {}", part_1_result);

    let part_2_result = input.lines().filter_map(|line| {
        let (low, high, letter, password) = parse_line(line)?;

        ((password.chars().nth(low-1)? == letter) ^ (password.chars().nth(high-1)? == letter)).then_some(())
    }).count();

    println!("Part 2: {}", part_2_result);

    Ok(())
}
