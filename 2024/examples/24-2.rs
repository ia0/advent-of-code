use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{Result, bail, ensure};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Node {
    A([u8; 3]),
    B(u8, u8),
}

impl FromStr for Node {
    type Err = anyhow::Error;
    fn from_str(x: &str) -> Result<Self> {
        let x: [u8; 3] = x.as_bytes().try_into()?;
        Ok(match x[0] {
            b'x' ..= b'z' => Node::B(x[0], std::str::from_utf8(&x[1 ..])?.parse()?),
            _ => Node::A(x),
        })
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::A(x) => write!(f, "{}", std::str::from_utf8(x).unwrap()),
            Node::B(x, y) => write!(f, "{}{y:02}", *x as char),
        }
    }
}

fn swap(gates: &mut HashMap<Node, (Node, Gate, Node)>, a: Node, b: Node) {
    let tmp = gates[&a];
    gates.insert(a, gates[&b]);
    gates.insert(b, tmp);
}

fn compute(
    values: &mut HashMap<Node, bool>, gates: &HashMap<Node, (Node, Gate, Node)>, node: Node,
    visited: &mut HashSet<Node>,
) -> Option<bool> {
    if !visited.insert(node) {
        return None;
    }
    if let Some(&value) = values.get(&node) {
        return Some(value);
    }
    let (a, g, b) = *gates.get(&node)?;
    let a = compute(values, gates, a, visited)?;
    let b = compute(values, gates, b, visited)?;
    let value = match g {
        Gate::And => a & b,
        Gate::Or => a | b,
        Gate::Xor => a ^ b,
    };
    assert!(values.insert(node, value).is_none());
    Some(value)
}

fn fuzz(gates: &HashMap<Node, (Node, Gate, Node)>, max: u8) -> Option<HashMap<Node, bool>> {
    let mask = (1 << (max + 1)) - 1;
    for _ in 0 .. 1000 {
        let x: u64 = rand::thread_rng().gen_range(0 .. 1 << (max + 1));
        let y: u64 = rand::thread_rng().gen_range(0 .. 1 << (max + 1));
        let mut values = HashMap::<Node, bool>::new();
        for i in 0 ..= max {
            assert!(values.insert(Node::B(b'x', i), x & (1 << i) != 0).is_none());
            assert!(values.insert(Node::B(b'y', i), y & (1 << i) != 0).is_none());
        }
        let mut z = 0;
        for i in 0 ..= max {
            let Some(b) = compute(&mut values, gates, Node::B(b'z', i), &mut HashSet::new()) else {
                return Some(values);
            };
            z |= (b as u64) << i;
        }
        if (x + y) & mask != z {
            assert_eq!(((x + y) ^ z) & ((1 << max) - 1), 0);
            return Some(values);
        }
    }
    None
}

fn remove(gates: &HashMap<Node, (Node, Gate, Node)>, node: Node, candidates: &mut HashSet<Node>) {
    if !candidates.remove(&node) {
        return;
    }
    let (a, _, b) = gates[&node];
    remove(gates, a, candidates);
    remove(gates, b, candidates);
}

fn find(
    gates: &HashMap<Node, (Node, Gate, Node)>, pos: u8, values: &HashMap<Node, bool>,
    candidates: &HashSet<Node>,
) -> Option<(Node, Node)> {
    let mut todo = vec![Node::B(b'z', pos)];
    let mut gates_mut = gates.clone();
    while let Some(node) = todo.pop() {
        if matches!(node, Node::B(b'x' | b'y', _)) {
            continue;
        }
        for &candidate in candidates {
            swap(&mut gates_mut, node, candidate);
            if fuzz(&gates_mut, pos).is_none() {
                return Some((node, candidate));
            }
            swap(&mut gates_mut, node, candidate);
        }
        let (a, g, b) = gates_mut[&node];
        match (values[&node], g, values[&a], values[&b]) {
            (true, Gate::And, true, true) => todo.extend_from_slice(&[a, b]),
            (false, Gate::And, true, false) => todo.push(b),
            (false, Gate::And, false, true) => todo.push(a),
            (false, Gate::And, false, false) => (),
            (true, Gate::Or, true, true) => (),
            (true, Gate::Or, true, false) => todo.push(a),
            (true, Gate::Or, false, true) => todo.push(b),
            (false, Gate::Or, false, false) => todo.extend_from_slice(&[a, b]),
            (_, Gate::Xor, _, _) => todo.extend_from_slice(&[a, b]),
            _ => unreachable!(),
        }
    }
    None
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut max = [0; 3]; // x y z
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if let Node::B(i, v) = line.split_once(": ").unwrap().0.parse::<Node>()? {
            max[(i - b'x') as usize] = std::cmp::max(max[(i - b'x') as usize], v);
        }
    }
    let mut gates = HashMap::<Node, (Node, Gate, Node)>::new();
    for line in lines {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        ensure!(words.len() == 5);
        ensure!(words[3] == "->");
        let node1 = words[0].parse()?;
        let node2 = words[2].parse()?;
        let node = words[4].parse()?;
        let gate = match words[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            x => bail!("bad gate {x}"),
        };
        if let Node::B(i, v) = node {
            max[(i - b'x') as usize] = std::cmp::max(max[(i - b'x') as usize], v);
        }
        assert!(gates.insert(node, (node1, gate, node2)).is_none());
    }
    assert!(max[1] == max[0]);
    assert!(max[2] == max[0] + 1);
    let mut swaps = Vec::new();
    let mut candidates: HashSet<Node> = gates.keys().copied().collect();
    for pos in 0 ..= max[2] {
        remove(&gates, Node::B(b'z', pos), &mut candidates);
        let Some(values) = fuzz(&gates, pos) else { continue };
        let (a, b) = find(&gates, pos, &values, &candidates).unwrap();
        swaps.push(format!("{a}"));
        swaps.push(format!("{b}"));
        swap(&mut gates, a, b);
        if swaps.len() == 8 {
            break;
        }
    }
    swaps.sort();
    writeln!(output, "{}", swaps.join(","))?;
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "cph,jqn,kwb,qkf,tgr,z12,z16,z24\n");
