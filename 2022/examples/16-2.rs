use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

struct Valve {
    flow: usize,
    tunnels: Vec<usize>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    minutes: usize,
    name: usize,
}

#[derive(Debug, Clone)]
struct State {
    pressure: usize,
    players: [Player; 2],
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
    todo.push(State {
        pressure: 0,
        players: [Player { minutes: 26, name: start }; 2],
        open: HashSet::new(),
    });
    let mut best = 0;
    while let Some(cur) = todo.pop() {
        best = std::cmp::max(best, cur.pressure);
        let remaining_flow = (0 .. names.len())
            .filter(|x| valves[x].flow > 0 && !cur.open.contains(x))
            .map(|x| valves[&x].flow)
            .sum::<usize>();
        let max_minutes = cur.players.iter().map(|x| x.minutes).max().unwrap();
        if cur.pressure + max_minutes * remaining_flow <= best {
            continue;
        }
        for name in 0 .. names.len() {
            if valves[&name].flow == 0 || cur.open.contains(&name) {
                continue;
            }
            let mut best = None;
            for player in 0 .. 2 {
                let minutes = cur.players[player].minutes;
                let time = dist[&cur.players[player].name][&name] + 1;
                if let Some(minutes) = minutes.checked_sub(time) {
                    if !matches!(best, Some((x, _)) if x > minutes) {
                        best = Some((minutes, player));
                    }
                }
            }
            let (minutes, player) = match best {
                Some(x) => x,
                None => continue,
            };
            let mut next = cur.clone();
            next.players[player].name = name;
            next.players[player].minutes = minutes;
            assert!(next.open.insert(name));
            next.pressure += next.players[player].minutes * valves[&name].flow;
            todo.push(next);
        }
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "2775\n");
