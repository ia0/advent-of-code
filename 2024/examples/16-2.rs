use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::rc::Rc;

use adventofcode::{Coord, EAST};
use anyhow::Result;

enum List {
    Nil,
    Cons(Coord, Rc<List>),
}

struct State {
    cost: i64,
    pos: Coord,
    dir: Coord,
    path: Rc<List>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            match byte {
                b'#' => assert!(walls.insert(pos)),
                b'S' => assert!(start.replace(pos).is_none()),
                b'E' => assert!(end.replace(pos).is_none()),
                b'.' => (),
                _ => unreachable!(),
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    let mut total = HashSet::new();
    let mut best = i64::MAX;
    let mut todo = BinaryHeap::new();
    todo.push(State { cost: 0, pos: start, dir: EAST, path: Rc::new(List::Nil) });
    let mut visited = HashMap::new();
    while let Some(State { cost, pos, dir, mut path }) = todo.pop() {
        if best < cost {
            continue;
        }
        match visited.entry((pos, dir)) {
            Entry::Occupied(x) if *x.get() < cost => continue,
            Entry::Occupied(_) => (),
            Entry::Vacant(x) => drop(x.insert(cost)),
        }
        path = Rc::new(List::Cons(pos, path));
        if pos == end {
            best = cost;
            while let List::Cons(pos, next) = &*path {
                total.insert(*pos);
                path = next.clone();
            }
            continue;
        }
        for dir in [dir.left(), dir.right()] {
            todo.push(State { cost: cost + 1000, pos, dir, path: path.clone() });
        }
        if !walls.contains(&(pos + dir)) {
            todo.push(State { cost: cost + 1, pos: pos + dir, dir, path });
        }
    }
    writeln!(output, "{}", total.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "433\n");
