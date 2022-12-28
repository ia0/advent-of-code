use std::cell::Cell;
use std::collections::HashMap;
use std::io::BufRead;

type Map = HashMap<u8, HashMap<u8, Cell<u8>>>;

fn add(graph: &mut Map, x: u8, y: u8) {
    *graph
        .entry(x)
        .or_insert(HashMap::new())
        .entry(y)
        .or_insert(Cell::new(0))
        .get_mut() += 1;
}

fn explore(graph: &Map, x: u8) -> usize {
    let mut max = 0;
    if let Some(children) = graph.get(&x) {
        for (&y, c) in children.iter() {
            if c.get() == 0 {
                continue;
            }
            let d = &graph[&y][&x];
            c.set(c.get() - 1);
            d.set(d.get() - 1);
            let current = x as usize + y as usize + explore(graph, y);
            c.set(c.get() + 1);
            d.set(d.get() + 1);
            max = std::cmp::max(max, current);
        }
    }
    max
}

fn main() {
    let stdin = std::io::stdin();
    let mut graph = HashMap::new();
    for line in stdin.lock().lines() {
        let edge: Vec<u8> = line.unwrap()
            .split('/')
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(edge.len(), 2);
        add(&mut graph, edge[0], edge[1]);
        add(&mut graph, edge[1], edge[0]);
    }
    println!("{}", explore(&graph, 0));
}
