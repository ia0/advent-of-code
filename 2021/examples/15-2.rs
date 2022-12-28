use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Coord = adventofcode::Coord<i64>;

fn solve(map: &HashMap<Coord, i64>, frame: Coord) -> i64 {
    let end = frame - Coord { x: 1, y: 1 };
    let mut done = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), Coord { x: 0, y: 0 }));
    loop {
        let (Reverse(risk), pos) = todo.pop().unwrap();
        if pos == end {
            return risk;
        }
        if !done.insert(pos) {
            continue;
        }
        for d in adventofcode::ADJACENT_PLUS {
            let q = pos + d;
            if let Some(r) = map.get(&q) {
                todo.push((Reverse(risk + r), q));
            }
        }
    }
}

fn main() {
    let input = File::open("examples/15.txt").unwrap();
    let map: Vec<Vec<u8>> =
        BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()).collect();
    assert!(map.iter().all(|x| x.len() == map[0].len()));
    let frame = Coord { x: map[0].len() as i64, y: map.len() as i64 };
    let map: HashMap<Coord, i64> = frame
        .iter()
        .map(|p| {
            let b = (map[p.y as usize][p.x as usize] - b'0') as i64;
            (Coord { x: 5, y: 5 }).iter().map(move |d| (frame * d + p, (b + d.x + d.y - 1) % 9 + 1))
        })
        .flatten()
        .collect();
    println!("{}", solve(&map, frame * 5));
}
