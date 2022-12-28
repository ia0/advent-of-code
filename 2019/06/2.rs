use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::io::BufRead;

const fn id(x: &[u8; 3]) -> usize {
    (x[2] as usize) << 16 | (x[1] as usize) << 8 | (x[0] as usize)
}

const YOU: usize = id(b"YOU");
const SAN: usize = id(b"SAN");

struct Node {
    name: usize,
    dist: usize,
}

fn find(neighbors: &HashMap<usize, HashSet<usize>>, src: usize, dst: usize) -> usize {
    let mut todo = VecDeque::new();
    let mut done = HashSet::new();
    todo.push_back(Node { name: src, dist: 0 });
    done.insert(src);
    while let Some(next) = todo.pop_front() {
        if next.name == dst {
            return next.dist;
        }
        for &child in neighbors.get(&next.name).unwrap() {
            if done.insert(child) {
                todo.push_back(Node { name: child, dist: next.dist + 1 });
            }
        }
    }
    unreachable!();
}

fn main() {
    let mut neighbors: HashMap<usize, HashSet<usize>> = HashMap::new();
    let stdin = std::io::stdin();
    let mut you = None;
    let mut san = None;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let a = id(<&[u8; 3]>::try_from(&line.as_bytes()[.. 3]).unwrap());
        let b = id(<&[u8; 3]>::try_from(&line.as_bytes()[4 ..]).unwrap());
        match b {
            YOU => {
                assert!(you.is_none());
                you = Some(a);
            }
            SAN => {
                assert!(san.is_none());
                san = Some(a);
            }
            _ => (),
        }
        assert!(neighbors.entry(a).or_default().insert(b));
        assert!(neighbors.entry(b).or_default().insert(a));
    }
    println!("{}", find(&neighbors, you.unwrap(), san.unwrap()));
}
