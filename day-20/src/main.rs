#![feature(str_split_once)]

use std::{collections::HashMap, error::Error, fs::read_to_string};

use ndarray::{Array1, Array2, s, Axis};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

type TileRef = usize;
type TileContent = Array2<char>;
type EdgeSlice = Array1<char>;
type EdgeToTileMap = HashMap<EdgeSlice, Vec<TileRef>>;

#[derive(Debug)]
struct Tile {
    id: TileRef,
    content: TileContent,
    left: EdgeSlice,
    right: EdgeSlice,
    top: EdgeSlice,
    bottom: EdgeSlice,
}

impl Tile {
    fn from_str(input: &str) -> Self {
        let (header, content) = input.split_once(":\n").expect("invalid entry");
        let content = TileContent::from_shape_vec(
            (10, 10), 
            content.lines().flat_map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<char>>()
        ).expect("invalid dimensions");
        let top = content.index_axis(Axis(0), 0).to_owned();
        let bottom = content.index_axis(Axis(0), 9).to_owned();
        let left = content.index_axis(Axis(1), 0).to_owned();
        let right = content.index_axis(Axis(1), 9).to_owned();

        Tile {
            id: header[5..].parse().expect("invalid id"),
            content, left, right, top, bottom
        }
    }

    #[inline]
    fn reverse_edge(edge: &EdgeSlice) -> EdgeSlice {
        // edge.slice(s![..;-1]).to_owned()
        edge.iter().cloned().rev().collect()
    }
}

fn find_corner_tiles(tiles: &HashMap<TileRef, Tile>) -> (Vec<TileRef>, EdgeToTileMap) {
    let mut edge_to_tile_ids: EdgeToTileMap = HashMap::new();
    let mut tile_to_num_corners: HashMap<TileRef, usize> = HashMap::new();

    for tile in tiles.values() {
        let lr = Tile::reverse_edge(&tile.left);
        let rr = Tile::reverse_edge(&tile.right);
        let br = Tile::reverse_edge(&tile.bottom);
        let tr = Tile::reverse_edge(&tile.top);
        let l = Tile::reverse_edge(&lr);
        let r = Tile::reverse_edge(&rr);
        let b = Tile::reverse_edge(&br);
        let t = Tile::reverse_edge(&tr);

        edge_to_tile_ids.entry(l).or_default().push(tile.id);
        edge_to_tile_ids.entry(r).or_default().push(tile.id);
        edge_to_tile_ids.entry(t).or_default().push(tile.id);
        edge_to_tile_ids.entry(b).or_default().push(tile.id);
        edge_to_tile_ids.entry(lr).or_default().push(tile.id);
        edge_to_tile_ids.entry(rr).or_default().push(tile.id);
        edge_to_tile_ids.entry(br).or_default().push(tile.id);
        edge_to_tile_ids.entry(tr).or_default().push(tile.id);
    }

    edge_to_tile_ids.iter()
        .filter(|(_, v)| v.len() == 1)
        .for_each(|(_, v)| *tile_to_num_corners.entry(v[0]).or_default() += 1);

    let corner_tile_ids = tile_to_num_corners.iter()
        .filter(|(_id, count)| **count == 4)
        .map(|(id, _)| *id)
        .collect::<Vec<usize>>();
    
    (corner_tile_ids, edge_to_tile_ids)
}

fn determine_starting_tile(tiles: &HashMap<usize, Tile>, corner_tile_ids: &[TileRef], edge_to_tile_ids: &EdgeToTileMap) -> TileRef {
    let mut unchanged_edge_position_to_tile_ids: HashMap<(usize, usize, usize, usize), Vec<TileRef>> = HashMap::new();

    for id in corner_tile_ids {
        let position_counts = (
            edge_to_tile_ids[&tiles[&id].left].len(),
            edge_to_tile_ids[&tiles[&id].right].len(),
            edge_to_tile_ids[&tiles[&id].top].len(),
            edge_to_tile_ids[&tiles[&id].bottom].len(),
        );
        unchanged_edge_position_to_tile_ids.entry(position_counts).or_default().push(*id);
    }
    
    unchanged_edge_position_to_tile_ids.iter()
        .filter(|(_, ids)| ids.len() == 1)
        .map(|(_, ids)| ids[0])
        .next().expect("ambiguous layout")
}

fn main() -> Result<()> {
    let input = read_to_string("input/input1.txt")?;

    let tiles: HashMap<usize, Tile> = input.split("\n\n").map(|tile_entry| Tile::from_str(tile_entry)).map(|t| (t.id, t)).collect();
    let (corner_tile_ids, edge_to_tile_ids) = find_corner_tiles(&tiles); 
    
    println!("Part 1: {} ({:?})", corner_tile_ids.iter().product::<usize>(), corner_tile_ids);

    // 1789 has L=1, R=2, T=1, B=2
    // 1187 has L=1, R=2, T=1, B=2
    // 3121 has L=2, R=1, T=1, B=2 => top right edge
    // 1889 has L=1, R=2, T=1, B=2

    println!("{}", determine_starting_tile(&tiles, &corner_tile_ids, &edge_to_tile_ids));

    Ok(())
}
