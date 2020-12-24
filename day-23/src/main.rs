use std::{error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Cup = usize;
#[derive(Debug)]
struct Cups {
    current: Cup,
    links: Vec<Cup>,
}

impl Cups {
    fn from_input(input: &str, length: usize) -> Self {
        let cups: Vec<Cup> = input
            .chars().map(|c| c.to_digit(10).unwrap() as usize)
            .chain(input.len()+1..=length)
            .collect();

        let mut links: Vec<Cup> = vec![0; cups.len() + 1];

        for (&cup, &next_cup) in cups.iter().zip(cups.iter().cycle().skip(1)).take(length) {
            links[cup] = next_cup;
        }
        
        Cups{ current: cups[0], links }
    }

    fn advance_round(&mut self) {
        let first_pickup = self.links[self.current];
        let second_pickup = self.links[first_pickup];
        let third_pickup = self.links[second_pickup];
        let new_head = self.links[third_pickup];

        self.links[self.current] = new_head;

        let mut destination = self.current.saturating_sub(1);

        loop {
            if destination == 0 { 
                destination = self.links.len() - 1;
            }

            if destination == first_pickup || destination == second_pickup || destination == third_pickup {
                destination = destination.saturating_sub(1);
                continue;
            }

            break;
        }

        self.links[third_pickup] = self.links[destination];
        self.links[second_pickup] = third_pickup;
        self.links[first_pickup] = second_pickup;
        self.links[destination] = first_pickup;

        self.current = new_head;
    }
}

fn main() -> Result<()> {

    let input = "219748365";
    // let input = "389125467"; // sample

    let mut cups = Cups::from_input(&input, 9);

    for _round in 0..100 {
        cups.advance_round();
    }

    println!(
        "Part 1: {}",
        cups.links.iter().take(cups.links.len() - 2).fold((1, "".to_string()), |(idx, mut result), _| {
            let next = cups.links[idx];
            result.push_str(next.to_string().as_str());
            (next, result)
        }).1
    );

    let mut cups = Cups::from_input(&input, 1_000_000);

    for _round in 0..10_000_000 {
        cups.advance_round();
    }

    println!("Part 2: {} ({} * {})", cups.links[1] * cups.links[cups.links[1]], cups.links[1], cups.links[cups.links[1]]);

    Ok(())
}
