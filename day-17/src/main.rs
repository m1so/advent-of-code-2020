use std::{collections::{HashMap, HashSet}, error::Error, fmt::Debug, fs::read_to_string, hash::Hash};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type ActiveCoordinates<T> = HashSet<T>;
type CoordinateCounts<T> = HashMap<T, usize>;

trait Offsetable<'a> {
    fn offset(&self, delta: &Self) -> Self; 
    fn get_offsets() -> &'a [Self] where Self: Sized;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate3D<T>(T, T, T);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate4D<T>(T, T, T, T);

impl<'a> Offsetable<'a> for Coordinate3D<i64> {
    fn offset(&self, delta: &Self) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1, self.2 + delta.2)
    }

    fn get_offsets() -> &'a [Coordinate3D<i64>] {
        &[
            Coordinate3D::<i64>(-1, -1, 0), Coordinate3D::<i64>(-1, 0, 0), Coordinate3D::<i64>(0, -1, 0), 
            Coordinate3D::<i64>(1, 1, 0), Coordinate3D::<i64>(1, 0, 0), Coordinate3D::<i64>(0, 1, 0), 
            Coordinate3D::<i64>(-1, 1, 0), Coordinate3D::<i64>(1, -1, 0),

            Coordinate3D::<i64>(-1, -1, 1), Coordinate3D::<i64>(-1, 0, 1), Coordinate3D::<i64>(0, -1, 1), 
            Coordinate3D::<i64>(1, 1, 1), Coordinate3D::<i64>(1, 0, 1), Coordinate3D::<i64>(0, 1, 1), 
            Coordinate3D::<i64>(-1, 1, 1), Coordinate3D::<i64>(1, -1, 1),

            Coordinate3D::<i64>(-1, -1, -1), Coordinate3D::<i64>(-1, 0, -1), Coordinate3D::<i64>(0, -1, -1), 
            Coordinate3D::<i64>(1, 1, -1), Coordinate3D::<i64>(1, 0, -1), Coordinate3D::<i64>(0, 1, -1), 
            Coordinate3D::<i64>(-1, 1, -1), Coordinate3D::<i64>(1, -1, -1),

            Coordinate3D::<i64>(0, 0, 1), Coordinate3D::<i64>(0, 0, -1),
        ]
    }
}

impl<'a> Offsetable<'a> for Coordinate4D<i64> {
    fn offset(&self, delta: &Self) -> Self {
        Self(self.0 + delta.0, self.1 + delta.1, self.2 + delta.2, self.3 + delta.3)
    }

    fn get_offsets() -> &'a [Coordinate4D<i64>] {
        &[
            Coordinate4D::<i64>(-1, -1, 0, 0), Coordinate4D::<i64>(-1, 0, 0, 0), Coordinate4D::<i64>(0, -1, 0, 0), 
            Coordinate4D::<i64>(1, 1, 0, 0), Coordinate4D::<i64>(1, 0, 0, 0), Coordinate4D::<i64>(0, 1, 0, 0), 
            Coordinate4D::<i64>(-1, 1, 0, 0), Coordinate4D::<i64>(1, -1, 0, 0),
            Coordinate4D::<i64>(-1, -1, 1, 0), Coordinate4D::<i64>(-1, 0, 1, 0), Coordinate4D::<i64>(0, -1, 1, 0), 
            Coordinate4D::<i64>(1, 1, 1, 0), Coordinate4D::<i64>(1, 0, 1, 0), Coordinate4D::<i64>(0, 1, 1, 0), 
            Coordinate4D::<i64>(-1, 1, 1, 0), Coordinate4D::<i64>(1, -1, 1, 0),
            Coordinate4D::<i64>(-1, -1, -1, 0), Coordinate4D::<i64>(-1, 0, -1, 0), Coordinate4D::<i64>(0, -1, -1, 0), 
            Coordinate4D::<i64>(1, 1, -1, 0), Coordinate4D::<i64>(1, 0, -1, 0), Coordinate4D::<i64>(0, 1, -1, 0), 
            Coordinate4D::<i64>(-1, 1, -1, 0), Coordinate4D::<i64>(1, -1, -1, 0),
            Coordinate4D::<i64>(0, 0, 1, 0), Coordinate4D::<i64>(0, 0, -1, 0),

            Coordinate4D::<i64>(-1, -1, 0, 1), Coordinate4D::<i64>(-1, 0, 0, 1), Coordinate4D::<i64>(0, -1, 0, 1), 
            Coordinate4D::<i64>(1, 1, 0, 1), Coordinate4D::<i64>(1, 0, 0, 1), 
            Coordinate4D::<i64>(0, 1, 0, 1), Coordinate4D::<i64>(-1, 1, 0, 1), Coordinate4D::<i64>(1, -1, 0, 1),
            Coordinate4D::<i64>(-1, -1, 1, 1), Coordinate4D::<i64>(-1, 0, 1, 1), Coordinate4D::<i64>(0, -1, 1, 1), 
            Coordinate4D::<i64>(1, 1, 1, 1), Coordinate4D::<i64>(1, 0, 1, 1), Coordinate4D::<i64>(0, 1, 1, 1), 
            Coordinate4D::<i64>(-1, 1, 1, 1), Coordinate4D::<i64>(1, -1, 1, 1),
            Coordinate4D::<i64>(-1, -1, -1, 1), Coordinate4D::<i64>(-1, 0, -1, 1), Coordinate4D::<i64>(0, -1, -1, 1),
            Coordinate4D::<i64>(1, 1, -1, 1), Coordinate4D::<i64>(1, 0, -1, 1), Coordinate4D::<i64>(0, 1, -1, 1),
            Coordinate4D::<i64>(-1, 1, -1, 1), Coordinate4D::<i64>(1, -1, -1, 1),
            Coordinate4D::<i64>(0, 0, 1, 1), Coordinate4D::<i64>(0, 0, -1, 1),

            Coordinate4D::<i64>(-1, -1, 0, -1), Coordinate4D::<i64>(-1, 0, 0, -1), Coordinate4D::<i64>(0, -1, 0, -1), 
            Coordinate4D::<i64>(1, 1, 0, -1), Coordinate4D::<i64>(1, 0, 0, -1), Coordinate4D::<i64>(0, 1, 0, -1), 
            Coordinate4D::<i64>(-1, 1, 0, -1), Coordinate4D::<i64>(1, -1, 0, -1),
            Coordinate4D::<i64>(-1, -1, 1, -1), Coordinate4D::<i64>(-1, 0, 1, -1), Coordinate4D::<i64>(0, -1, 1, -1), 
            Coordinate4D::<i64>(1, 1, 1, -1), Coordinate4D::<i64>(1, 0, 1, -1), Coordinate4D::<i64>(0, 1, 1, -1), 
            Coordinate4D::<i64>(-1, 1, 1, -1), Coordinate4D::<i64>(1, -1, 1, -1),
            Coordinate4D::<i64>(-1, -1, -1, -1), Coordinate4D::<i64>(-1, 0, -1, -1), Coordinate4D::<i64>(0, -1, -1, -1), 
            Coordinate4D::<i64>(1, 1, -1, -1), Coordinate4D::<i64>(1, 0, -1, -1), Coordinate4D::<i64>(0, 1, -1, -1), 
            Coordinate4D::<i64>(-1, 1, -1, -1), Coordinate4D::<i64>(1, -1, -1, -1),
            Coordinate4D::<i64>(0, 0, 1, -1), Coordinate4D::<i64>(0, 0, -1, -1),

            Coordinate4D::<i64>(0, 0, 0, 1), Coordinate4D::<i64>(0, 0, 0, -1),
        ]
    }
}

fn get_coordinate_counts<'a, C>(active_coords: &ActiveCoordinates<C>) -> CoordinateCounts<C> 
where C: 'a + Debug + Hash + Eq + Offsetable<'a>{
    let mut coordinate_counts = CoordinateCounts::new();

    for coord in active_coords {
        for delta in C::get_offsets() {
            let offset_coord = coord.offset(delta);
            *coordinate_counts.entry(offset_coord).or_insert(0) += 1;
        }
    }

    coordinate_counts
}

fn simulate_dimension<'a, C>(mut active_coords: ActiveCoordinates<C>, n_cycles: usize) -> ActiveCoordinates<C> 
where C: 'a + Debug + Copy + Hash + Eq + Offsetable<'a> {
    for _cycle in 0..n_cycles {
        let coordinate_counts = get_coordinate_counts(&active_coords);

        active_coords = coordinate_counts
            .iter()
            .filter_map(|(coord, count)| {
                match (count, active_coords.contains(coord)) {
                    (2, true) | (3, _) => Some(*coord),
                    _ => None,
                }
            }).collect();
    }

    active_coords
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let active_coords_3d = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| {
                match c {
                    '#' => Some(Coordinate3D::<i64>(x as i64, y as i64, 0 as i64)),
                    _ => None,
                }
            }).collect::<ActiveCoordinates<Coordinate3D<i64>>>()
        }).collect();
    
    let active_coords_4d = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(|(x, c)| {
                match c {
                    '#' => Some(Coordinate4D::<i64>(x as i64, y as i64, 0 as i64, 0 as i64)),
                    _ => None,
                }
            }).collect::<ActiveCoordinates<Coordinate4D<i64>>>()
        }).collect();

    println!("Part 1: {}", simulate_dimension(active_coords_3d, 6).len());
    println!("Part 2: {}", simulate_dimension(active_coords_4d, 6).len());

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample_1() {
        // .#.
        // ..#
        // ###
        let mut active_coords: ActiveCoordinates<Coordinate3D<i64>> = ActiveCoordinates::new();
        active_coords.insert(Coordinate3D::<i64>(1, 0, 0));
        active_coords.insert(Coordinate3D::<i64>(2, 1, 0));
        active_coords.insert(Coordinate3D::<i64>(0, 2, 0));
        active_coords.insert(Coordinate3D::<i64>(1, 2, 0));
        active_coords.insert(Coordinate3D::<i64>(2, 2, 0));

        assert_eq!(simulate_dimension(active_coords, 6).len(), 112);
    }

    #[test]
    fn test_part_2_sample_1() {
        // .#.
        // ..#
        // ###
        let mut active_coords: ActiveCoordinates<Coordinate4D<i64>> = ActiveCoordinates::new();
        active_coords.insert(Coordinate4D::<i64>(1, 0, 0, 0));
        active_coords.insert(Coordinate4D::<i64>(2, 1, 0, 0));
        active_coords.insert(Coordinate4D::<i64>(0, 2, 0, 0));
        active_coords.insert(Coordinate4D::<i64>(1, 2, 0, 0));
        active_coords.insert(Coordinate4D::<i64>(2, 2, 0, 0));

        assert_eq!(simulate_dimension(active_coords, 6).len(), 848);
    }
}
