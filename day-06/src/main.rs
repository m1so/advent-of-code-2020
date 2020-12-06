use std::{collections::HashSet, error::Error, collections::HashMap, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/input1.txt")?;

    let questions_count: usize = input.split("\n\n").map(|group_entry| {
        group_entry.replace("\n", "").chars().collect::<HashSet<char>>().len()
    }).sum();

    println!("Part 1: {}", questions_count);

    let questions_count: usize = input.split("\n\n").map(|group_entry| {
        let mut question_counts: HashMap<char, usize> = HashMap::new();
        let mut num_entries: usize = 0;

        for line in group_entry.lines() {
            num_entries += 1;
            for char in line.chars() {
                *question_counts.entry(char).or_insert(0) += 1
            }
        }
        
        question_counts.iter().filter(|(_, v)| *v == &num_entries).count()
    }).sum();

    println!("Part 2: {}", questions_count);

    Ok(())
}
