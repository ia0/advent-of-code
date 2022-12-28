use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Portal([u8; 2]),
}

fn parse() -> HashMap<Coord, Tile> {
    let input = File::open("examples/20.txt").unwrap();
    let mut map = HashMap::new();
    let mut portals = HashMap::new();
    for (y, line) in BufReader::new(input).lines().enumerate() {
        for (x, c) in line.unwrap().into_bytes().iter().enumerate() {
            let coord = Coord { x: x as i64, y: y as i64 };
            let tile = match c {
                b' ' => continue,
                b'.' => Tile::Empty,
                b'#' => Tile::Wall,
                c @ b'A' ..= b'Z' => {
                    assert!(portals.insert(coord, *c).is_none());
                    continue;
                }
                _ => panic!(),
            };
            assert!(map.insert(coord, tile).is_none());
        }
    }
    for (&pos_a, &a) in &portals {
        for &step in &[Coord { x: 0, y: 1 }, Coord { x: 1, y: 0 }] {
            let pos_b = pos_a + step;
            let b = match portals.get(&pos_b) {
                None => continue,
                Some(x) => *x,
            };
            for &coord in &[pos_b + step, pos_a - step] {
                if let Some(Tile::Empty) = map.get(&coord) {
                    map.insert(coord, Tile::Portal([a, b]));
                }
            }
        }
    }
    map
}

fn find_best(map: HashMap<Coord, Tile>, portals: HashMap<[u8; 2], Vec<Coord>>) -> usize {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct State {
        dist: usize,
        pos: Coord,
    }
    let mut visited = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State { dist: 0, pos: portals[b"AA"][0] }));
    loop {
        let Reverse(State { dist, pos }) = todo.pop().unwrap();
        if !visited.insert(pos) {
            continue;
        }
        if pos == portals[b"ZZ"][0] {
            return dist;
        }
        let portal = match map[&pos] {
            Tile::Portal(portal) => {
                let portals = &portals[&portal];
                if portals.len() == 1 {
                    None
                } else {
                    Some(if portals[0] == pos { portals[1] } else { portals[0] })
                }
            }
            _ => None,
        };
        for next in adventofcode::ADJACENT_PLUS.iter().map(|&step| pos + step).chain(portal) {
            if map.get(&next).map_or(true, |&x| x == Tile::Wall) {
                continue;
            }
            if visited.contains(&next) {
                continue;
            }
            todo.push(Reverse(State { dist: dist + 1, pos: next }));
        }
    }
}

fn main() {
    let map = parse();
    let mut portals: HashMap<[u8; 2], Vec<Coord>> = HashMap::new();
    for (&coord, &tile) in &map {
        if let Tile::Portal(portal) = tile {
            portals.entry(portal).or_default().push(coord);
        }
    }
    println!("{}", find_best(map, portals));
}
