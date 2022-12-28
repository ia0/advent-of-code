use std::collections::HashSet;
use std::io::BufRead;

const LIMIT: i32 = 10000;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, p: Point) -> i32 {
        (p.x - self.x).abs() + (p.y - self.y).abs()
    }
}

fn dist(points: &[Point], p: Point) -> i32 {
    points.iter().map(|q| q.dist(p)).sum()
}

fn main() {
    let stdin = std::io::stdin();
    let mut points = Vec::new();
    for line in stdin.lock().lines() {
        let xy: Vec<i32> = line.unwrap().split(", ").map(|x| x.parse().unwrap()).collect();
        assert_eq!(xy.len(), 2);
        points.push(Point { x: xy[0], y: xy[1] });
    }
    let mut region = HashSet::new();
    for &point in &points {
        let mut todo = Vec::new();
        todo.push(point);
        while let Some(p) = todo.pop() {
            if region.contains(&p) {
                continue;
            }
            if dist(&points, p) >= LIMIT {
                continue;
            }
            region.insert(p);
            todo.push(Point { x: p.x - 1, y: p.y });
            todo.push(Point { x: p.x + 1, y: p.y });
            todo.push(Point { x: p.x, y: p.y - 1 });
            todo.push(Point { x: p.x, y: p.y + 1 });
        }
    }
    println!("{}", region.len());
}
