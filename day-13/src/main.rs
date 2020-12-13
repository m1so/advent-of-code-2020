use std::{error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let mut lines = input.lines();

    let departure_ts = lines.next().ok_or("missing input")?.parse::<u64>()?;
    let timetable = lines.next().ok_or("missing input")?;
    
    let mut bus_waits: Vec<(u64, u64)> = timetable
        .split(",")
        .filter(|part| *part != "x")
        .map(|part| {
            let id = part.parse::<u64>().expect("invalid id");
            let quotient = (departure_ts as f64 / (id as f64)).ceil() as u64;
            let earliest_departure_ts = id * quotient;
            (id, earliest_departure_ts - departure_ts)
        })
        .collect();
    
    bus_waits.sort_unstable_by(|(_, x), (_, y)| x.cmp(y));

    println!("Part 1: {}", bus_waits[0].0 * bus_waits[0].1);

    let mut bus_offsets: Vec<(u64, u64)> = timetable
        .split(",")
        .enumerate()
        .filter(|(_, part)| *part != "x")
        .map(|(offset, part)| (part.parse::<u64>().expect("invalid id"), offset as u64))
        .collect();
    
    let mut t: u64 = 0;
    let mut n: u64 = bus_offsets.remove(0).0; // product of candidate n_i (bus ids) in x + a_i ≡ 0 (mod n_i)

    loop {
        t += n;

        for (idx, (bus_id, offset)) in bus_offsets.iter().enumerate() {
            if (t + offset).rem_euclid(*bus_id) == 0 {
                n *= *bus_id;
                bus_offsets.remove(idx);
                
                break;
            }
        }

        if bus_offsets.len() == 0 { break; }
    }


    println!("Part 2: {}", t);

    Ok(())
}
