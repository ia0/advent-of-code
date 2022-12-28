use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;

const MODULO: usize = 20183;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Cave {
    erosion: HashMap<Coord, usize>,
    depth: usize,
}

impl Cave {
    fn new(depth: usize, target: Coord) -> Cave {
        let mut erosion = HashMap::new();
        erosion.insert(Coord { x: 0, y: 0 }, depth % MODULO);
        erosion.insert(target, depth % MODULO);
        Cave { erosion, depth }
    }

    fn get(&mut self, coord: Coord) -> usize {
        if let Some(erosion) = self.erosion.get(&coord) {
            return *erosion;
        }
        let geologic;
        if coord.y == 0 {
            geologic = (coord.x * 16807) % MODULO;
        } else if coord.x == 0 {
            geologic = (coord.y * 48271) % MODULO;
        } else {
            geologic = self.get(Coord { x: coord.x - 1, y: coord.y })
                * self.get(Coord { x: coord.x, y: coord.y - 1 })
                % MODULO;
        }
        let erosion = (geologic + self.depth) % MODULO;
        self.erosion.insert(coord, erosion);
        erosion
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    coord: Coord,
    tool: usize,
}

#[derive(Clone, Copy, Debug)]
struct Todo {
    dist: usize,
    pos: Pos,
}

impl std::cmp::PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl std::cmp::Eq for Todo {}
impl std::cmp::Ord for Todo {
    fn cmp(&self, other: &Todo) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}
impl std::cmp::PartialOrd for Todo {
    fn partial_cmp(&self, other: &Todo) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let depth = lines.next().unwrap().unwrap();
    let target = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());
    let depth: usize = depth.split_whitespace().nth(1).unwrap().parse().unwrap();
    let target: Vec<usize> =
        target.split_whitespace().nth(1).unwrap().split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(target.len(), 2);
    let target = Coord { x: target[0], y: target[1] };

    let mut cave = Cave::new(depth, target);
    let mut visited = HashMap::new();
    let mut todo = BinaryHeap::new();
    todo.push(Todo { dist: 0, pos: Pos { coord: Coord { x: 0, y: 0 }, tool: 1 } });
    while let Some(Todo { dist, pos }) = todo.pop() {
        let Pos { coord, tool } = pos;
        let Coord { x, y } = coord;
        let region = cave.get(pos.coord) % 3;
        if region == tool {
            continue;
        }
        if pos == (Pos { coord: target, tool: 1 }) {
            println!("{}", dist);
            return;
        }
        match visited.entry(pos) {
            Entry::Vacant(entry) => {
                entry.insert(dist);
            }
            Entry::Occupied(entry) => {
                assert!(dist >= *entry.get());
                continue;
            }
        }
        if pos.coord.x > 0 {
            todo.push(Todo { dist: dist + 1, pos: Pos { coord: Coord { x: x - 1, y: y }, tool } });
        }
        if pos.coord.y > 0 {
            todo.push(Todo { dist: dist + 1, pos: Pos { coord: Coord { x: x, y: y - 1 }, tool } });
        }
        todo.push(Todo { dist: dist + 1, pos: Pos { coord: Coord { x: x + 1, y: y }, tool } });
        todo.push(Todo { dist: dist + 1, pos: Pos { coord: Coord { x: x, y: y + 1 }, tool } });
        todo.push(Todo { dist: dist + 7, pos: Pos { coord, tool: 3 - region - tool } });
    }
}
