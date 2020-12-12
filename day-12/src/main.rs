#![feature(or_patterns)]

use std::{error::Error, fs::read_to_string};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

static DIRECTION_ORDER: &[Action] = &[Action::North, Action::East, Action::South, Action::West];

struct Navigation {
    heading: Action,
    x: i32,
    y: i32,
    waypoint: (i32, i32),
    is_waypoint_active: bool,
}

struct Instruction {
    action: Action,
    value: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Instruction {
    fn parse(input: &str) -> Result<Self> {
        let mut parts = input.chars();
        let action = Action::parse(parts.next().ok_or("missing operation")?)?;
        let value = parts.collect::<String>().parse::<usize>()?;

        Ok(Self { action, value })
    }
}

impl Action {
    fn parse(input: char) -> Result<Self> {
        match input {
            'N' => Ok(Action::North),
            'S' => Ok(Action::South),
            'E' => Ok(Action::East),
            'W' => Ok(Action::West),
            'L' => Ok(Action::Left),
            'R' => Ok(Action::Right),
            'F' => Ok(Action::Forward),
            _ => Err("unknown operation".into()),
        }
    }

    fn order(&self) -> usize {
        DIRECTION_ORDER.iter().position(|action| action == self).ok_or("not a direction").unwrap()
    }
}

impl Navigation {
    fn new() -> Self {
        Self{ heading: Action::East, x: 0, y: 0, waypoint: (0, 0), is_waypoint_active: false}
    }

    fn with_waypoint(x: i32, y: i32) -> Self {
        Self{ heading: Action::East, x: 0, y: 0, waypoint: (x, y), is_waypoint_active: true}
    }

    fn handle(&mut self, instruction: &Instruction) {
        if self.is_waypoint_active {
            self._handle_with_waypoint(instruction)
        } else {
            self._handle_ship(instruction)
        }
    }

    fn _handle_ship(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction{action: Action::Forward, value} => {
                match self.heading {
                    Action::North => self.y += *value as i32,
                    Action::South => self.y -= *value as i32,
                    Action::East => self.x += *value as i32,
                    Action::West => self.x -= *value as i32,
                    _ => panic!("invalid heading"),
                }
            },
            Instruction{action: action @ (Action::Left | Action::Right), value} => {
                let direction_changes = *value as i32 / 90 * if *action == Action::Left { -1 } else { 1 };
                let normalized_changes = direction_changes.rem_euclid(4) as usize;
                self.heading = DIRECTION_ORDER[(self.heading.order() + normalized_changes).rem_euclid(4)];
            },
            Instruction{action: Action::North, value} => self.y += *value as i32,
            Instruction{action: Action::South, value} => self.y -= *value as i32,
            Instruction{action: Action::West, value} => self.x -= *value as i32,
            Instruction{action: Action::East, value} => self.x += *value as i32,
        };
    }

    fn _handle_with_waypoint(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction{action: Action::Forward, value} => {
                self.x += self.waypoint.0 * *value as i32;
                self.y += self.waypoint.1 * *value as i32;
            },
            Instruction{action: action @ (Action::Left | Action::Right), value} => {
                let direction_changes = *value as i32 / 90 * if *action == Action::Left { -1 } else { 1 };
                let normalized_changes = direction_changes.rem_euclid(4) as usize;
                match normalized_changes {
                    1 => self.waypoint = (self.waypoint.1, self.waypoint.0 * -1),
                    2 => self.waypoint = (self.waypoint.0 * -1, self.waypoint.1 * -1),
                    3 => self.waypoint = (self.waypoint.1 * -1, self.waypoint.0),
                    _ => (),
                };
            },
            Instruction{action: Action::East, value} => self.waypoint = (self.waypoint.0 + *value as i32, self.waypoint.1),
            Instruction{action: Action::West, value} => self.waypoint = (self.waypoint.0 - *value as i32, self.waypoint.1),
            Instruction{action: Action::South, value} => self.waypoint = (self.waypoint.0, self.waypoint.1 - *value as i32),
            Instruction{action: Action::North, value} => self.waypoint = (self.waypoint.0, self.waypoint.1 + *value as i32),
        };
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;
    let mut navigation = Navigation::new();
    let mut waypoint_navigation = Navigation::with_waypoint(10, 1);

    for line in input.lines() {
        let instruction = Instruction::parse(line)?;
        navigation.handle(&instruction);
        waypoint_navigation.handle(&instruction);
    }
    
    println!("Part 1: {}", navigation.x.abs() + navigation.y.abs());
    println!("Part 2: {}", waypoint_navigation.x.abs() + waypoint_navigation.y.abs());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        let mut navigation = Navigation::new();

        navigation.handle(&Instruction::parse("F10").unwrap());
        assert_eq!(navigation.x, 10);
        assert_eq!(navigation.y, 0);

        navigation.handle(&Instruction::parse("N3").unwrap());
        assert_eq!(navigation.x, 10);
        assert_eq!(navigation.y, 3);

        navigation.handle(&Instruction::parse("F7").unwrap());
        assert_eq!(navigation.x, 17);
        assert_eq!(navigation.y, 3);

        navigation.handle(&Instruction::parse("R90").unwrap());
        assert_eq!(navigation.x, 17);
        assert_eq!(navigation.y, 3);

        navigation.handle(&Instruction::parse("F11").unwrap());
        assert_eq!(navigation.x, 17);
        assert_eq!(navigation.y, -8);
    }

    #[test]
    fn test_part_2_sample_1() {
        let mut navigation = Navigation::with_waypoint(10, 1);

        navigation.handle(&Instruction::parse("F10").unwrap());
        assert_eq!(navigation.x, 100);
        assert_eq!(navigation.y, 10);
        assert_eq!(navigation.waypoint, (10, 1));

        navigation.handle(&Instruction::parse("N3").unwrap());
        assert_eq!(navigation.x, 100);
        assert_eq!(navigation.y, 10);
        assert_eq!(navigation.waypoint, (10, 4));

        navigation.handle(&Instruction::parse("F7").unwrap());
        assert_eq!(navigation.x, 170);
        assert_eq!(navigation.y, 38);
        assert_eq!(navigation.waypoint, (10, 4));

        navigation.handle(&Instruction::parse("R90").unwrap());
        assert_eq!(navigation.x, 170);
        assert_eq!(navigation.y, 38);
        assert_eq!(navigation.waypoint, (4, -10));

        navigation.handle(&Instruction::parse("F11").unwrap());
        assert_eq!(navigation.x, 214);
        assert_eq!(navigation.y, -72);
        assert_eq!(navigation.waypoint, (4, -10));
    }
}
