use std::collections::HashMap;
use std::io::BufRead;

const MODULO: usize = 20183;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
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

    let mut erosion = HashMap::new();
    for x in 0 ..= target.x {
        assert!(erosion.insert(Coord { x, y: 0 }, (x * 16807 + depth) % MODULO).is_none());
    }
    for y in 1 ..= target.y {
        assert!(erosion.insert(Coord { x: 0, y }, (y * 48271 + depth) % MODULO).is_none());
    }
    for x in 1 ..= target.x {
        for y in 1 ..= target.y {
            let a = *erosion.get(&Coord { x: x - 1, y: y }).unwrap();
            let b = *erosion.get(&Coord { x: x, y: y - 1 }).unwrap();
            assert!(erosion.insert(Coord { x, y }, (a * b + depth) % MODULO).is_none());
        }
    }
    assert!(erosion.insert(target, depth % MODULO).is_some());

    println!("{}", erosion.values().map(|x| x % 3).sum::<usize>());
}
