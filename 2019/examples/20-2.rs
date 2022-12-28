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
        level: usize,
        pos: Coord,
    }
    let mut visited = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State { dist: 0, level: 0, pos: portals[b"AA"][0] }));
    loop {
        let Reverse(State { dist, level, pos }) = todo.pop().unwrap();
        if !visited.insert((level, pos)) {
            continue;
        }
        if level == 0 && pos == portals[b"ZZ"][0] {
            return dist;
        }
        let portal = match map[&pos] {
            Tile::Portal(portal) => {
                let portals = &portals[&portal];
                if portals.len() == 1 {
                    None
                } else {
                    if portals[0] == pos {
                        if level > 0 {
                            Some((level - 1, portals[1]))
                        } else {
                            None
                        }
                    } else {
                        Some((level + 1, portals[0]))
                    }
                }
            }
            _ => None,
        };
        for next in
            adventofcode::ADJACENT_PLUS.iter().map(|&step| (level, pos + step)).chain(portal)
        {
            if map.get(&next.1).map_or(true, |&x| x == Tile::Wall) {
                continue;
            }
            if visited.contains(&next) {
                continue;
            }
            todo.push(Reverse(State { dist: dist + 1, level: next.0, pos: next.1 }));
        }
    }
}

fn main() {
    let map = parse();
    let mut portals: HashMap<[u8; 2], Vec<Coord>> = HashMap::new();
    let mut max = Coord { x: 2, y: 2 };
    for (&coord, &tile) in &map {
        max = std::cmp::max(max, coord);
        if let Tile::Portal(portal) = tile {
            portals.entry(portal).or_default().push(coord);
        }
    }
    let is_outer = |p: Coord| p.x == 2 || p.y == 2 || p.x == max.x || p.y == max.y;
    for (portal, coords) in &mut portals {
        if coords.len() == 1 {
            assert!(portal == b"AA" || portal == b"ZZ");
            assert!(is_outer(coords[0]));
            continue;
        }
        assert_eq!(coords.len(), 2);
        if is_outer(coords[1]) {
            coords.swap(0, 1);
        }
        assert!(is_outer(coords[0]));
    }
    println!("{}", find_best(map, portals));
}
