use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

struct Valve {
    flow: usize,
    tunnels: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    pressure: usize, // total
    minutes: usize,  // remaining
    name: usize,
    open: HashSet<usize>,
}

impl State {
    fn key(&self) -> usize {
        self.pressure
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
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
        matches!(self.cmp(other), Ordering::Equal)
    }
}

#[derive(Default)]
struct Names(Vec<String>);

impl Names {
    fn get_or_insert(&mut self, name: &str) -> usize {
        match self.0.iter().position(|x| x == name) {
            Some(x) => x,
            None => {
                self.0.push(name.to_string());
                self.0.len() - 1
            }
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut names = Names::default();
    let mut valves = HashMap::new();
    let regex =
        regex::Regex::new(r"^Valve (..) has flow rate=(.*); tunnels? leads? to valves? (.*)$")?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let captures = regex.captures(&line).unwrap();
        let name = names.get_or_insert(&captures[1]);
        let flow = captures[2].parse()?;
        let tunnels = captures[3].split(", ").map(|x| names.get_or_insert(x)).collect();
        assert!(valves.insert(name, Valve { flow, tunnels }).is_none());
    }
    let start = names.get_or_insert("AA");
    let mut dist = HashMap::<usize, HashMap<usize, usize>>::new();
    for x in 0 .. names.len() {
        let mut ys = HashMap::new();
        for &y in &valves[&x].tunnels {
            ys.insert(y, 1);
        }
        ys.insert(x, 0);
        dist.insert(x, ys);
    }
    for k in 0 .. names.len() {
        for x in 0 .. names.len() {
            let xk = match dist[&x].get(&k) {
                Some(x) => *x,
                None => continue,
            };
            for y in 0 .. names.len() {
                let ky = match dist[&k].get(&y) {
                    Some(x) => *x,
                    None => continue,
                };
                let d = xk + ky;
                let v = dist.get_mut(&x).unwrap().entry(y).or_insert(d);
                *v = std::cmp::min(*v, d);
            }
        }
    }
    for x in 0 .. names.len() {
        if x != start && valves[&x].flow == 0 {
            dist.remove(&x);
        } else {
            dist.get_mut(&x).unwrap().retain(|y, _| valves[y].flow > 0 && x != *y);
        }
    }
    let mut todo = BinaryHeap::new();
    todo.push(State { pressure: 0, minutes: 30, name: start, open: HashSet::new() });
    let mut best = 0;
    while let Some(cur) = todo.pop() {
        best = std::cmp::max(best, cur.pressure);
        for (&name, &time) in &dist[&cur.name] {
            let time = time + 1;
            if cur.minutes < time || cur.open.contains(&name) {
                continue;
            }
            let mut next = cur.clone();
            next.name = name;
            assert!(next.open.insert(name));
            next.minutes -= time;
            next.pressure += next.minutes * valves[&name].flow;
            todo.push(next);
        }
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "2124\n");
