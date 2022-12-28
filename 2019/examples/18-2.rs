use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Key(u8),
    Door(u8),
}

struct Parse {
    map: HashMap<Coord, Tile>,
    pos: Coord,
}

fn parse() -> Parse {
    let input = File::open("examples/18.txt").unwrap();
    // let input = File::open("test-3.txt").unwrap();
    let mut map = HashMap::new();
    let mut pos = None;
    for (y, line) in BufReader::new(input).lines().enumerate() {
        for (x, c) in line.unwrap().into_bytes().iter().enumerate() {
            let coord = Coord { x, y };
            let tile = match c {
                b'.' => Tile::Empty,
                b'#' => Tile::Wall,
                b'@' => {
                    pos = Some(coord);
                    Tile::Empty
                }
                c @ b'a' ..= b'z' => Tile::Key(c - b'a'),
                c @ b'A' ..= b'Z' => Tile::Door(c - b'A'),
                _ => panic!(),
            };
            assert!(map.insert(coord, tile).is_none());
        }
    }
    let pos = pos.unwrap();
    Parse { map, pos }
}

fn adjacent(pos: Coord) -> Vec<Coord> {
    let mut adjacent = Vec::new();
    adjacent.push(Coord { x: pos.x + 1, y: pos.y });
    adjacent.push(Coord { x: pos.x, y: pos.y + 1 });
    if pos.x > 0 {
        adjacent.push(Coord { x: pos.x - 1, y: pos.y });
    }
    if pos.y > 0 {
        adjacent.push(Coord { x: pos.x, y: pos.y - 1 });
    }
    adjacent
}

fn find_best(mut map: HashMap<Coord, Tile>, pos: Coord) -> usize {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct State {
        dist: usize,
        focus: Option<usize>,
        pos: [Coord; 4],
        found: u32,
    }
    map.insert(pos, Tile::Wall);
    for pos in adjacent(pos) {
        map.insert(pos, Tile::Wall);
    }
    let all_keys = map
        .iter()
        .filter_map(|(_, &tile)| if let Tile::Key(key) = tile { Some(key) } else { None })
        .fold(0, |r, x| r | 1 << x);
    let mut visited = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State {
        focus: None,
        dist: 0,
        pos: [
            Coord { x: pos.x - 1, y: pos.y - 1 },
            Coord { x: pos.x + 1, y: pos.y - 1 },
            Coord { x: pos.x + 1, y: pos.y + 1 },
            Coord { x: pos.x - 1, y: pos.y + 1 },
        ],
        found: 0,
    }));
    loop {
        let Reverse(State { focus, dist, pos, found }) = todo.pop().unwrap();
        if !visited.insert((pos, found)) {
            continue;
        }
        if found == all_keys {
            return dist;
        }
        for i in 0 .. 4 {
            if focus.map_or(false, |x| i != x) {
                continue;
            }
            for next in adjacent(pos[i]) {
                let new_found = match map[&next] {
                    Tile::Empty => found,
                    Tile::Key(key) => found | 1 << key,
                    Tile::Door(door) if found & 1 << door == 0 => continue,
                    Tile::Door(_) => found,
                    Tile::Wall => continue,
                };
                let mut pos = pos;
                pos[i] = next;
                if visited.contains(&(pos, new_found)) {
                    continue;
                }
                let new_focus = if new_found == found { focus.or(Some(i)) } else { None };
                todo.push(Reverse(State {
                    focus: new_focus,
                    dist: dist + 1,
                    pos,
                    found: new_found,
                }));
            }
        }
    }
}

fn main() {
    let Parse { map, pos } = parse();
    println!("{}", find_best(map, pos));
}
