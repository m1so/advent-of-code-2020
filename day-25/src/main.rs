fn find_loop_size(public_key: u64, subject_number: u64) -> u64 {
    let mut value: u64 = 1;
    let mut loop_size: u64 = 0;

    while value != public_key {
        loop_size += 1;
        value = (value * subject_number) % 20201227;
    }

    loop_size
}

fn compute_encryption_key(subject_number: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1_u64, |value, _| (value * subject_number) % 20201227)
}

fn main() {
    let card_pk = 19774466_u64;
    let door_pk = 7290641_u64;

    println!("Part 1: {}", compute_encryption_key(card_pk, find_loop_size(door_pk, 7)));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let subject_number = 7_u64;

        let card_pk = 5764801_u64;
        let card_loop_size = 8_u64;

        let door_pk = 17807724_u64;
        let door_loop_size = 11_u64;

        let encryption_key = 14897079_u64;

        assert_eq!(find_loop_size(card_pk, subject_number), card_loop_size);
        assert_eq!(find_loop_size(door_pk, subject_number), door_loop_size);
        assert_eq!(compute_encryption_key(subject_number, card_loop_size), card_pk);
        assert_eq!(compute_encryption_key(subject_number, door_loop_size), door_pk);

        assert_eq!(compute_encryption_key(door_pk, card_loop_size), encryption_key);
        assert_eq!(compute_encryption_key(card_pk, door_loop_size), encryption_key);
    }
}
