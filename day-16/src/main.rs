use std::{error::Error, fs::read_to_string};
use parse_display::{Display, FromStr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{name}: {lower.0}-{lower.1} or {upper.0}-{upper.1}")]
struct Field {
    name: String,
    #[from_str(default)]
    lower: (u32, u32),
    #[from_str(default)]
    upper: (u32, u32),
}

impl Field {
    fn is_within_bounds(&self, number: u32) -> bool {
        (number <= self.lower.1 && number >= self.lower.0) || (number <= self.upper.1 && number >= self.upper.0)
    }
}

fn calculate_scanning_error_rate(nearby_tickets: Vec<Vec<u32>>, rules: &[Field]) -> u32 {
    nearby_tickets.iter()
        .filter_map(|values| {
            values.iter().find(|value| !rules.iter().any(|rule| rule.is_within_bounds(**value)))
        })
        .sum()
}

fn find_column_names(nearby_tickets: Vec<Vec<u32>>, rules: &[Field]) -> Vec<String> {
    let valid_tickets: Vec<Vec<u32>> = nearby_tickets.iter()
        .filter_map(|values| {
            match values.iter().find(|value| !rules.iter().any(|rule| rule.is_within_bounds(**value))) {
                None => Some(values.clone()),
                Some(_) => None,
            }
        })
        .collect();
    
    let columnar_values: Vec<Vec<u32>> = (0..valid_tickets[0].len())
        .map(|i| valid_tickets.iter().map(|inner| inner[i]).collect())
        .collect();
    
    let mut candidate_fields: Vec<(usize, Vec<String>)> = vec![(0, vec![]); columnar_values.len()];

    for rule in rules {
        for (idx, column) in columnar_values.iter().enumerate() {
            if column.iter().all(|value| rule.is_within_bounds(*value)) {
                candidate_fields[idx].0 = idx;
                candidate_fields[idx].1.push(rule.name.clone());
            }
        }
    }

    candidate_fields.sort_by(|(_, a), (_, b)| a.len().cmp(&b.len()));


    for i in 0..candidate_fields.len() {
        if candidate_fields[i].1.len() != 1 { continue; }
        
        for j in 0..candidate_fields.len() {
            if i == j { continue; };
            
            if let Some(position) = candidate_fields[j].1.iter().position(|other_name| *other_name == *candidate_fields[i].1[0]) {
                let other = &mut candidate_fields[j].1;
                other.remove(position);
            }
        }
    }

    let mut column_names: Vec<Option<String>> = vec![None; columnar_values.len()];

    for (target_idx, values) in candidate_fields {
        assert!(values.len() == 1);
        column_names[target_idx] = Some(values[0].clone());
    }

    column_names.iter().filter_map(|v| v.clone()).collect()
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Field> = parts[0].lines()
        .map(|line| line.parse::<Field>().unwrap())
        .collect();
    
    let my_ticket: Vec<u32> = parts[1].lines().skip(1).next().ok_or("invalid ticket")?
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    
    let nearby_tickets: Vec<Vec<u32>> = parts[2].lines()
        .skip(1)
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    println!("Part 1: {}", calculate_scanning_error_rate(nearby_tickets.clone(), &rules));

    let part2: u64 = find_column_names(nearby_tickets.clone(), &rules).iter().enumerate().filter_map(|(idx, name)| {
        match name.starts_with("departure") {
            true => Some(my_ticket[idx] as u64),
            false => None,
        }
    }).product();

    println!("Part 2: {}", part2);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        let nearby_tickets = vec![
            vec![7,3,47],
            vec![40,4,50],
            vec![55,2,20],
            vec![38,6,12],
        ];

        let rules: Vec<Field> = vec![
            "class: 1-3 or 5-7".parse().unwrap(),
            "row: 6-11 or 33-44".parse().unwrap(),
            "seat: 13-40 or 45-50".parse().unwrap(),
        ];

        assert_eq!(calculate_scanning_error_rate(nearby_tickets, &rules), 71);
    }

    #[test]
    fn test_part_2_sample_1() {
        let nearby_tickets = vec![
            vec![7,3,47],
            vec![40,4,50],
            vec![55,2,20],
            vec![38,6,12],
        ];

        let rules: Vec<Field> = vec![
            "class: 1-3 or 5-7".parse().unwrap(),
            "row: 6-11 or 33-44".parse().unwrap(),
            "seat: 13-40 or 45-50".parse().unwrap(),
        ];

        let field_names = find_column_names(nearby_tickets, &rules);

        assert_eq!(field_names.get(0), Some(&"row".to_string()));
        assert_eq!(field_names.get(1), Some(&"class".to_string()));
        assert_eq!(field_names.get(2), Some(&"seat".to_string()));
    }
}

