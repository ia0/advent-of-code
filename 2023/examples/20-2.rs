use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, ensure, Context, Result};

#[derive(Default)]
struct Problem {
    nodes: HashMap<String, Node>,
    pulse: [i64; 2],
}

struct Node {
    kind: Kind,
    dests: Vec<String>,
}

enum Kind {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { highs: HashMap<String, bool> },
}

impl Problem {
    fn init(&mut self) {
        let mut inputs = HashMap::<String, HashSet<String>>::new();
        for (src, node) in &self.nodes {
            for dst in &node.dests {
                inputs.entry(dst.to_string()).or_default().insert(src.to_string());
            }
        }
        for (name, node) in self.nodes.iter_mut() {
            if let Kind::Conjunction { highs } = &mut node.kind {
                *highs = inputs[name].iter().map(|x| (x.clone(), false)).collect();
            }
        }
    }

    fn reset(&mut self) {
        for node in self.nodes.values_mut() {
            match &mut node.kind {
                Kind::Broadcast => (),
                Kind::FlipFlop { on } => *on = false,
                Kind::Conjunction { highs } => highs.values_mut().for_each(|x| *x = false),
            }
        }
    }

    fn push(&mut self, end: &str) -> bool {
        let mut done = false;
        let mut todo = VecDeque::new();
        todo.push_back(("button".to_string(), false, "broadcaster".to_string()));
        while let Some((src, high, name)) = todo.pop_front() {
            self.pulse[high as usize] += 1;
            done |= name == end && !high;
            let node = match self.nodes.get_mut(&name) {
                Some(x) => x,
                None => continue,
            };
            let high = match &mut node.kind {
                Kind::Broadcast => high,
                Kind::FlipFlop { .. } if high => continue,
                Kind::FlipFlop { on } => {
                    *on = !*on;
                    *on
                }
                Kind::Conjunction { highs } => {
                    *highs.get_mut(&src).unwrap() = high;
                    highs.values().any(|x| !x)
                }
            };
            for dest in &node.dests {
                todo.push_back((name.clone(), high, dest.clone()));
            }
        }
        done
    }

    fn min(&mut self, end: &str) -> i64 {
        self.reset();
        let mut total = 1;
        while !self.push(end) {
            total += 1;
        }
        total
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut problem = Problem::default();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (name, dests) = line.split_once(" -> ").context("no ->")?;
        let (kind, name) = if name == "broadcaster" {
            (Kind::Broadcast, name)
        } else if let Some(name) = name.strip_prefix('%') {
            (Kind::FlipFlop { on: false }, name)
        } else if let Some(name) = name.strip_prefix('&') {
            (Kind::Conjunction { highs: HashMap::new() }, name)
        } else {
            bail!("invalid name {name:?}");
        };
        let dests = dests.split(", ").map(|x| x.to_string()).collect();
        ensure!(problem.nodes.insert(name.to_string(), Node { kind, dests }).is_none());
    }
    problem.init();
    let total = ["sv", "ng", "ft", "jz"].iter().map(|x| problem.min(x)).product::<i64>();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "224602011344203\n");
