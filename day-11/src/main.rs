use std::{collections::HashMap, error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type Position = (i32, i32);
type SeatLayout = HashMap<Position, Seat>;

static OFFSETS: &[Position] = &[(-1, -1), (-1, 0), (0, -1), (1, 1), (1, 0), (0, 1), (-1, 1), (1, -1)];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Seat {
    Empty, // L
    Occupied, // #
    Floor, // .
}

#[derive(Debug)]
struct WaitingArea {
    seat_layout: SeatLayout,
    ray_length: usize, 
    occupied_tolerance: usize,
}

impl WaitingArea {
    fn new(seat_layout: SeatLayout, ray_length: usize, occupied_tolerance: usize) -> Self {
        Self{seat_layout, ray_length, occupied_tolerance}
    }

    fn advance_round(&mut self) -> bool {
        let mut new_layout = SeatLayout::new();

        for ((row, col), seat) in self.seat_layout.iter() {
            if *seat == Seat::Floor {
                new_layout.insert((*row, *col), Seat::Floor);
                continue;
            }

            let num_occupied_adjacent = self.count_neightbours(*row, *col);
            
            let new_seat = match seat {
                Seat::Empty if num_occupied_adjacent == 0 => Seat::Occupied,
                Seat::Occupied if num_occupied_adjacent >= self.occupied_tolerance => Seat::Empty,
                _ => *seat, 
            };

            new_layout.insert((*row, *col), new_seat);
        }

        let is_stabilized = new_layout.iter().all(|(k, v)| {
            self.seat_layout.contains_key(k) && self.seat_layout.get(k).unwrap() == v
        });

        self.seat_layout = new_layout;

        is_stabilized
    }

    #[inline]
    fn count_neightbours(&self, row: i32, column: i32) -> usize {
        let mut num_occupied_adjacent: usize = 0;

        for (row_offset, col_offset) in OFFSETS {
            for ray in 1..=self.ray_length {
                match self.seat_layout.get(
                    &(row + (row_offset * (ray as i32)), column + (col_offset * (ray as i32)))
                ) {
                    Some(Seat::Occupied) => {
                        num_occupied_adjacent += 1;
                        break;
                    },
                    Some(Seat::Floor) => continue,
                    Some(Seat::Empty) => break,
                    None => break,
                }
            }
        }

        num_occupied_adjacent
    }

    fn count_occupied(&self) -> usize {
        self.seat_layout.values().filter(|v| **v == Seat::Occupied).count()
    }

    #[allow(dead_code)]
    fn print_layout(&self) {
        let num_rows = self.seat_layout.keys().map(|(row, _)| row).max().unwrap();
        let num_cols = self.seat_layout.keys().map(|(_, col)| col).max().unwrap();

        for row in 0..=*num_rows {
            for col in 0..=*num_cols {
                match self.seat_layout.get(&(row, col)) {
                    Some(Seat::Occupied) => print!("#"),
                    Some(Seat::Empty) => print!("L"),
                    Some(Seat::Floor) => print!("."),
                    None => print!(" "),
                }
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let mut seat_layout = SeatLayout::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, position) in row.chars().enumerate() {
            let seat = match position {
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                '.' => Seat::Floor,
                _ => continue,
            };
            seat_layout.insert((row_idx as i32, col_idx as i32), seat);
        }
    }

    let mut waiting_area = WaitingArea::new(seat_layout.clone(), 1, 4);

    let count_occupied = loop {
        if waiting_area.advance_round() {
            break waiting_area.count_occupied();
        }
    };

    println!("Part 1: {}", count_occupied);

    let mut waiting_area = WaitingArea::new(seat_layout.clone(), width.max(height), 5);

    let count_occupied = loop {
        if waiting_area.advance_round() {
            break waiting_area.count_occupied();
        }
    };

    println!("Part 2: {}", count_occupied);

    Ok(())
}
