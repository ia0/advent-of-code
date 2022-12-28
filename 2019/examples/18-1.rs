use std::cmp::Ordering;
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

fn find_best(map: &HashMap<Coord, Tile>, pos: Coord) -> usize {
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct State {
        dist: usize,
        pos: Coord,
        found: u32,
    }
    impl Ord for State {
        fn cmp(&self, other: &State) -> Ordering {
            Ordering::Equal
                .then(self.dist.cmp(&other.dist).reverse())
                .then(self.pos.cmp(&other.pos))
                .then(self.found.cmp(&other.found))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &State) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    let all_keys = map
        .iter()
        .filter_map(|(_, &tile)| if let Tile::Key(key) = tile { Some(key) } else { None })
        .fold(0, |r, x| r | 1 << x);
    let mut visited = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(State { dist: 0, pos, found: 0 });
    loop {
        let State { dist, pos, found } = todo.pop().unwrap();
        if !visited.insert((pos, found)) {
            continue;
        }
        if found == all_keys {
            return dist;
        }
        let mut adjacent = Vec::new();
        adjacent.push(Coord { x: pos.x + 1, y: pos.y });
        adjacent.push(Coord { x: pos.x, y: pos.y + 1 });
        if pos.x > 0 {
            adjacent.push(Coord { x: pos.x - 1, y: pos.y });
        }
        if pos.y > 0 {
            adjacent.push(Coord { x: pos.x, y: pos.y - 1 });
        }
        for next in adjacent {
            let found = match map[&next] {
                Tile::Empty => found,
                Tile::Key(key) => found | 1 << key,
                Tile::Door(door) if found & 1 << door == 0 => continue,
                Tile::Door(_) => found,
                Tile::Wall => continue,
            };
            if visited.contains(&(next, found)) {
                continue;
            }
            todo.push(State { dist: dist + 1, pos: next, found: found });
        }
    }
}

fn main() {
    let Parse { map, pos } = parse();
    println!("{}", find_best(&map, pos));
}
