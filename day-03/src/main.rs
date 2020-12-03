#![feature(bool_to_option)]

use std::{error::Error, fs::File, io::BufReader, iter, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input/input1.txt")?;
    let reader = BufReader::new(file);
    let tree_char = '#';

    let tree_count = reader.lines().skip(1).enumerate().filter_map(|(idx, line)| {
        let row = line.ok()?;
        let offset = ((idx+1) * 3) % row.len();

        (row.chars().nth(offset)? == tree_char).then_some(())
    }).count();

    println!("Part 1: {}", tree_count);

    let file = File::open("input/input1.txt")?;
    let reader = BufReader::new(file);

    let offset_bases: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut matches: Vec<usize> = iter::repeat(0).take(offset_bases.len()).collect();

    for (idx, line) in reader.lines().skip(1).enumerate() {
        let row = line?;
        let row_len = row.len();
        let current_row = idx + 1; // since we skip the first row

        for (offset_idx, (row_peek, column_peek)) in offset_bases.iter().enumerate() {
            if current_row % row_peek != 0 { continue; }
            let offset = ((current_row / row_peek) * column_peek) % row_len;

            if row.chars().nth(offset).unwrap() == tree_char {
                matches[offset_idx] += 1;
            }
        }
    }

    println!("Part 2: {:#?}", matches);
    println!("Part 2: {}", matches.iter().fold(1, |a, b| a * b));

    Ok(())
}
