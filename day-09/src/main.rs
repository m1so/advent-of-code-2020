use std::{error::Error, fs::read_to_string};
use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let numbers = input.lines()
        .map(|line| line.parse::<usize>().map_err(|_| "invalid number".into()))
        .collect::<Result<Vec<usize>>>()?;
    
    let invalid_entry = find_invalid_entry(numbers.clone(), 25);
    println!("Part 1: {}", invalid_entry.ok_or("no invalid entry found")?);

    let weakness = find_encryption_weakness(numbers, 25)?;
    println!("Part 2: {}", weakness);

    Ok(())
}

fn find_invalid_entry(sequence: Vec<usize>, preamble_length: usize) -> Option<usize> {
    sequence
        .windows(preamble_length)
        .zip(sequence.iter().skip(preamble_length))
        .find_map(|(previous, candidate)| {
            if previous.iter().tuple_combinations().find(|(x, y)| *x + *y == *candidate).is_none() {
                Some(*candidate)
            } else {
                None
            }
        })
}

fn find_encryption_weakness(sequence: Vec<usize>, preamble_length: usize) -> Result<usize> {
    let invalid_entry = find_invalid_entry(sequence.clone(), preamble_length).ok_or("no invalid entry found")?;

    (2..sequence.len()).find_map(|window_size| {
        sequence.windows(window_size).find_map(|contiguous_block| {
            if contiguous_block.iter().sum::<usize>() != invalid_entry {
                return None;
            }
            Some(contiguous_block.iter().min().unwrap() + contiguous_block.iter().max().unwrap())
        })
    }).ok_or("encryption has no weakness".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<usize> = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let result = find_invalid_entry(input, 5);
        assert_eq!(result, Some(127));
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input: Vec<usize> = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let result = find_encryption_weakness(input, 5)?;
        assert_eq!(result, 62);
        Ok(())
    }
}
