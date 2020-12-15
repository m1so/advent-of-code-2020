use std::{collections::HashMap, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn recite(starting_numbers: Vec<usize>, n_turns: usize) -> usize {
    let mut starting_numbers = starting_numbers.clone();
    starting_numbers.reverse();

    let mut numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last_number: usize = 0;
    let mut next_number: usize = 0;

    for turn in 1..=n_turns {
        // Speak starting numbers
        if starting_numbers.len() > 0 {
            next_number = starting_numbers.pop().unwrap();
            numbers.entry(next_number).or_default().push(turn);
            last_number = next_number;
            continue;
        }

        // Check if the number has been spoken before
        let entry = numbers.entry(last_number).or_default();
        
        match entry.len() {
            0 | 1 => next_number = 0,
            n => next_number = entry[n-1] - entry[n-2],
        };

        // Bookkeeping
        numbers.entry(next_number).or_default().push(turn);

        last_number = next_number;
    }

    next_number
}

fn main() -> Result<()> {
    let starting_numbers: Vec<usize> = vec![15,5,1,4,7,0];

    println!("Part 1: {}", recite(starting_numbers.clone(), 2020));
    println!("Part 2: {}", recite(starting_numbers.clone(), 30000000));

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_samples() {
        assert_eq!(recite(vec![0,3,6], 2020), 436);
        assert_eq!(recite(vec![1,3,2], 2020), 1);
        assert_eq!(recite(vec![2,1,3], 2020), 10);
        assert_eq!(recite(vec![3,1,2], 2020), 1836);
    }

    #[test]
    fn test_part_2_samples() {
        assert_eq!(recite(vec![0,3,6], 30000000), 175594);
        assert_eq!(recite(vec![3,2,1], 30000000), 18);
    }
}
