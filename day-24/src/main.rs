#![feature(hash_drain_filter)]
use std::{collections::{BTreeMap, HashMap, HashSet}, error::Error, fs::read_to_string, iter::FromIterator};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type PathInner = BTreeMap<Direction, usize>;

static OPPOSITES: &[&[Direction]] = &[
    &[Direction::E, Direction::W],
    &[Direction::NE, Direction::SW],
    &[Direction::NW, Direction::SE],
];

static ALTERNATES: &[((Direction, Direction), Direction)] = &[
    ((Direction::NE, Direction::SE), Direction::E),
    ((Direction::NW, Direction::SW), Direction::W),
    ((Direction::E, Direction::NW), Direction::NE),
    ((Direction::E, Direction::SW), Direction::SE),
    ((Direction::W, Direction::NE), Direction::NW),
    ((Direction::W, Direction::SE), Direction::SW),
];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug)]
struct Directions { inner: Vec<Direction> }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Path { inner: PathInner }

#[derive(Debug)]
struct Floor { flips: HashMap<Path, usize> }

impl Directions {
    fn from_str(input: &str) -> Self {
        let mut head = 0;
        let mut directions: Vec<Direction> = Vec::new();

        while head < input.len() {
            let mut skip = 2;
            let direction = match &input[head..(head + 2).min(input.len())] {
                "ne" => Direction::NE,
                "nw" => Direction::NW,
                "se" => Direction::SE,
                "sw" => Direction::SW,
                _ => {
                    skip = 1;
                    match &input[head..head + 1] {
                        "e" => Direction::E,
                        "w" => Direction::W,
                        _ => panic!("invalid direction"),
                    }
                }
            };
            directions.push(direction);
            head += skip;
        }

        Self { inner: directions }
    }

    fn to_path(&self) -> Path {
        let mut counts = PathInner::new();

        for &direction in &self.inner {
            *counts.entry(direction).or_insert(0) += 1;
        }

        Path { inner: counts }
    }
}

impl Path {
    fn min_occurrences(&self, dirs: &[Direction]) -> usize {
        *dirs.iter().map(|dir| self.inner.get(dir).unwrap_or(&0)).min().unwrap_or(&0)
    }

    fn simplify(&mut self) {
        loop {
            let mut did_simplify = false;

            for &((first, second), simplified) in ALTERNATES {
                let min = self.min_occurrences(&[first, second]);
    
                if min > 0 {
                    *self.inner.get_mut(&first).unwrap() -= min;
                    *self.inner.get_mut(&second).unwrap() -= min;
                    *self.inner.entry(simplified).or_insert(0) += min;
                    did_simplify = true;
                }
            }
    
            for &path in OPPOSITES {
                let min = self.min_occurrences(path);
    
                if min > 0 {
                    path.iter().for_each(|dir| *self.inner.get_mut(dir).unwrap() -= min);
                    did_simplify = true;
                }
            }

            if !did_simplify { break; }
        }

        self.inner = self.inner.iter().filter_map(|(&dir , &count)| {
            if count > 0 {
                Some((dir, count))
            } else {
                None
            }
        }).collect();
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours: Vec<Self> = Vec::with_capacity(6);

        for &direction in &[Direction::E, Direction::W, Direction::NE, Direction::NW, Direction::SE, Direction::SW] {
            let mut new = self.clone();
            *new.inner.entry(direction).or_insert(0) += 1;
            new.simplify();
            neighbours.push(new);
        }

        neighbours
    }
}

impl Floor {
    fn from_str(input: &str) -> Self {
        let mut flips: HashMap<Path, usize> = HashMap::new();

        for line in input.lines() {
            let mut path = Directions::from_str(line).to_path();
            path.simplify();
            *flips.entry(path).or_insert(0) += 1;
        }

        Self { flips }
    }

    fn black_paths(&self) -> HashSet<Path> {
        let mut flips = self.flips.clone();
        let black: HashMap<Path, usize> = flips.drain_filter(|_, v| *v % 2 == 1).collect();
        HashSet::from_iter(black.keys().cloned())
    }

    fn white_paths(&self) -> HashSet<Path> {
        let mut flips = self.flips.clone();
        let white: HashMap<Path, usize> = flips.drain_filter(|_, v| *v % 2 == 0).collect();
        HashSet::from_iter(white.keys().cloned())
    }

    fn process_days(&self, n: usize) -> HashSet<Path> {
        let mut black_paths = self.black_paths();

        for _day in 0..n {
            let mut black_neighbours: HashMap<Path, usize> = HashMap::new();

            for path in black_paths.iter() {
                for neighbour in path.neighbours() {
                    *black_neighbours.entry(neighbour).or_insert(0) += 1;
                }
            }

            black_paths = black_neighbours.iter().filter_map(|(path, count)| {
                if black_paths.contains(path) && (*count == 1 || *count == 2) {
                    Some(path.clone())
                } else if !black_paths.contains(path) && *count == 2 {
                    Some(path.clone())
                } else {
                    None
                }
            }).collect();
        }

        black_paths
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let floor = Floor::from_str(&input);

    println!("Part 1: {}", floor.black_paths().len());
    println!("Part 2: {}", floor.process_days(100).len());

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    #[test]
    fn test_directions_are_simplified() {
        let mut path = Directions::from_str("nwwswee").to_path();
        path.simplify();

        assert!(path.inner.is_empty())
    }

    #[test]
    fn test_paths_equal() {
        let mut path1 = Directions::from_str("enewne").to_path();
        path1.simplify();

        let mut path2 = Directions::from_str("wnenee").to_path();
        path2.simplify();
        
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_part_1_sample() {
        let floor = Floor::from_str(&INPUT);

        assert_eq!(floor.black_paths().len(), 10);
        assert_eq!(floor.white_paths().len(), 5);
    }

    #[test]
    fn test_part_2_first_samples() {
        assert_eq!(Floor::from_str(&INPUT).process_days(1).len(), 15);
        assert_eq!(Floor::from_str(&INPUT).process_days(2).len(), 12);
        assert_eq!(Floor::from_str(&INPUT).process_days(3).len(), 25);
        assert_eq!(Floor::from_str(&INPUT).process_days(4).len(), 14);
    }

    #[test]
    fn test_part_2_last_sample() {
        assert_eq!(Floor::from_str(&INPUT).process_days(100).len(), 2208);
    }
}
