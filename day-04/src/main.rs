use std::{error::Error, fs::File, io::BufReader, collections::{HashSet, HashMap}, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    let required_fields: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();

    let mut reader = BufReader::new(File::open("input/input1.txt")?);
    let mut input: String = String::new();
    let _ = reader.read_to_string(&mut input);


    let num_valid_passports = input.split("\n\n").fold(0, |count, line| {
        let passport_fields: HashSet<&str> = line.split_whitespace().map(|pair| pair.split(':').next().unwrap()).collect();

        count + required_fields.is_subset(&passport_fields) as i32
    });

    println!("Part 1: {}", num_valid_passports);

    let num_valid_passports = input.split("\n\n").fold(0, |count, line| {
        let passport: HashMap<&str, &str> = line.split_whitespace().map(|pair| {
            let mut pair_iter = pair.split(':');
            (pair_iter.next().unwrap(), pair_iter.next().unwrap())
        }).collect();
        
        let passport_fields: HashSet<&str> = passport.keys().cloned().collect();
        count + (
            required_fields.is_subset(&passport_fields) && 
            passport.iter().all(|(k, v)| is_field_valid(*k, *v))
        ) as i32
    });

    println!("Part 2: {}", num_valid_passports);

    Ok(())
}



fn is_field_valid(key: &str, value: &str) -> bool {
    match key {
        "byr" => match value.parse::<usize>() {
            Ok(birth_year) => birth_year >= 1920 && birth_year <= 2002,
            Err(_) => false,
        },
        "iyr" => match value.parse::<usize>() {
            Ok(issue_year) => issue_year >= 2010 && issue_year <= 2020,
            Err(_) => false,
        },
        "eyr" => match value.parse::<usize>() {
            Ok(expiration_year) => expiration_year >= 2020 && expiration_year <= 2030,
            Err(_) => false,
        },
        "hgt" => {
            let unit = &value[value.len()-2..];
            let height: i32 = (&value[..value.len()-2]).parse::<i32>().unwrap_or(-1);

            match unit {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                _ => false,
            }
        },
        "hcl" => value.starts_with("#") && value.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => value.len() == 9 && value.parse::<usize>().is_ok(),
        "cid" => true,
        _ => unreachable!(),
    }
}
