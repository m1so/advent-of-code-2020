use std::{error::Error, fs::File, io::BufReader, io::prelude::*};

const ROWS: u32 = 256;
const COLUMNS: u32 = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open("input/input1.txt")?);
    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);

    let mut seat_ids: Vec<u32> = input.lines().map(|boarding_pass| gen_seat_id(boarding_pass)).collect();
    seat_ids.sort();
    
    let highest_seat_id = seat_ids.last().ok_or("empty input")?;
    println!("Part 1: {:#?}", highest_seat_id);

    let possible_seat_range = seat_ids.windows(2).find(|pair| pair[1] - pair[0] == 2).ok_or("seat not found")?;
    assert_eq!(possible_seat_range.len(), 2);
    let my_seat_id = possible_seat_range.iter().sum::<u32>() / possible_seat_range.len() as u32;
    println!("Part 2: {:#?}", my_seat_id);

    Ok(())
}

fn gen_seat_id(boarding_pass: &str) -> u32 {
    let row_encoding = u32::from_str_radix(boarding_pass[0..7].replace("B", "1").replace("F", "0").as_str(), 2).unwrap();
    let column_encoding = u32::from_str_radix(boarding_pass[7..].replace("R", "1").replace("L", "0").as_str(), 2).unwrap();
    
    ((ROWS-1) & row_encoding) * COLUMNS + ((COLUMNS-1) & column_encoding)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        assert_eq!(gen_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(gen_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(gen_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(gen_seat_id("BBFFBBFRLL"), 820);
    }
}
