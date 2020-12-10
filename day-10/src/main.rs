use std::{error::Error, fs::read_to_string};
use itertools::Itertools;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let ratings: Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();

    let counts = find_differences(ratings.clone());
    println!("Part 1: {} ({:?})", counts[0] * counts[2], counts);

    println!("Part 2: {}", count_combinations(ratings.clone()));

    Ok(())
}

fn find_differences(ratings: Vec<usize>) -> Vec<usize> {
    let differences: &mut [usize; 3] = &mut [0, 0, 1];
    
    let mut ratings = ratings.clone();
    ratings.push(0);
    ratings.sort_unstable();
    
    for (a, b) in ratings.iter().tuple_windows() {
        if let Some(diff) = differences.get_mut(b - a - 1) {
            *diff += 1;
        }
    }

    differences.iter().cloned().collect::<Vec<usize>>()
}


fn count_combinations(ratings: Vec<usize>) -> usize {
    let mut ratings = ratings.clone();
    ratings.push(0);
    ratings.sort_unstable();

    ratings.iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect::<Vec<_>>() // need to collect as TupleWindows doesn't have `split` method
        .split(|difference| *difference == 3) // break into max reaching spans
        .map(|difference_span| possible_steps_count(&difference_span))
        .product()
}

fn possible_steps_count(differences: &[usize]) -> usize {
    match differences[..] {
        [] | [1] | [3] => 1,
        [2] => 2,
        [1, 1] | [1, 2] | [2, 1] => 2,
        [2, 2] => 1,
        _ => {
            match differences[..2] {
                [1, 1] => {
                    let mut combined_case = vec![2];
                    combined_case.extend(&differences[2..]);
                    2 * possible_steps_count(&differences[2..]) + possible_steps_count(&combined_case[..])
                },
                [1, 2] | [2, 1] => {
                    let mut combined_case = vec![3];
                    combined_case.extend(&differences[2..]);
                    2 * possible_steps_count(&differences[2..]) + possible_steps_count(&combined_case[..])
                },
                _ => possible_steps_count(&differences[2..]),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        let input: Vec<usize> = vec![16,10,15,5,1,11,7,19,6,12,4];
        let result = find_differences(input.clone());
        assert_eq!(result, vec![7, 0, 5]);
    }

    #[test]
    fn test_part_1_sample_2() {
        let input: Vec<usize> = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        let result = find_differences(input.clone());
        assert_eq!(result, vec![22, 0, 10]);
    }

    #[test]
    fn test_possible_steps_count() {
        assert_eq!(possible_steps_count(&[1]), 1); // [1]
        assert_eq!(possible_steps_count(&[1, 1]), 2); // [1, 1], [2]
        assert_eq!(possible_steps_count(&[1, 1, 1]), 4); // [1, 1, 1], [2, 1], [1, 2], [3]

        // [1, 1, 1, 1], 
        // [1, 1, 2], [1, 2, 1], [2, 1, 1],
        // [1, 3], [3, 1], [2, 2]
        assert_eq!(possible_steps_count(&[1, 1, 1, 1]), 7);

        assert_eq!(possible_steps_count(&[1, 2, 1]), 3);
        assert_eq!(possible_steps_count(&[2, 1, 1]), 3);
    }

    #[test]
    fn test_part_2_sample_1() {
        let input: Vec<usize> = vec![16,10,15,5,1,11,7,19,6,12,4];
        let result = count_combinations(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_2_sample_2() {
        let input: Vec<usize> = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        let result = count_combinations(input);
        assert_eq!(result, 19208);
    }
}
