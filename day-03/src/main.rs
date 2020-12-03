#![feature(bool_to_option)]

use std::{error::Error, fs::File, io::BufReader, iter, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/input1.txt")?);

    let offset_bases: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut matches: Vec<usize> = iter::repeat(0).take(offset_bases.len()).collect();

    for (idx, line) in reader.lines().skip(1).enumerate() {
        let row = line?;
        let current_row = idx + 1; // since we skip the first row

        for (offset_idx, (row_peek, column_peek)) in offset_bases.iter().enumerate() {
            if current_row % row_peek != 0 { continue; }
            let offset = ((current_row / row_peek) * column_peek) % row.len();

            if row.chars().nth(offset).unwrap() == '#' {
                matches[offset_idx] += 1;
            }
        }
    }

    println!("Slopes: {:#?}", offset_bases);
    println!("Solution: {:#?} ({})", matches, matches.iter().fold(1, |a, b| a * b));

    Ok(())
}
