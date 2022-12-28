use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

#[derive(Clone, Copy)]
struct Coord {
    data: [i32; 4],
}

impl Coord {
    fn dist(self, other: Coord) -> u32 {
        let mut dist = 0;
        for i in 0 .. 4 {
            dist += (self.data[i] - other.data[i]).abs() as u32;
        }
        dist
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut points = Vec::new();
    for line in stdin.lock().lines() {
        let d: Vec<i32> = line.unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        points.push(Coord { data: [d[0], d[1], d[2], d[3]] });
    }
    let len = points.len();
    let mut edges = HashMap::new();
    for i in 0 .. len {
        for j in 0 .. len {
            if i == j || points[i].dist(points[j]) > 3 {
                continue;
            }
            assert!(edges.entry(i).or_insert_with(|| HashSet::new()).insert(j));
        }
    }
    let mut todo: HashSet<usize> = HashSet::from_iter(0 .. len);
    let mut constellations = 0;
    while !todo.is_empty() {
        constellations += 1;
        let mut stack = Vec::new();
        stack.push(*todo.iter().next().unwrap());
        while let Some(i) = stack.pop() {
            if !todo.remove(&i) {
                continue;
            }
            if let Some(edges) = edges.get(&i) {
                for &j in edges {
                    stack.push(j);
                }
            }
        }
    }
    println!("{}", constellations);
}
