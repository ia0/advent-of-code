use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, ensure, Result};

#[derive(Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

type Node = [u8; 3];

fn parse_node(x: &str) -> Result<Node> {
    Ok(x.as_bytes().try_into()?)
}

fn compute(
    values: &mut HashMap<Node, bool>, gates: &HashMap<Node, (Node, Gate, Node)>, node: Node,
) -> bool {
    if let Some(&value) = values.get(&node) {
        return value;
    }
    let (a, g, b) = gates[&node];
    let a = compute(values, gates, a);
    let b = compute(values, gates, b);
    match g {
        Gate::And => a & b,
        Gate::Or => a | b,
        Gate::Xor => a ^ b,
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut values = HashMap::<Node, bool>::new();
    let mut gates = HashMap::<Node, (Node, Gate, Node)>::new();
    let mut lines = BufReader::new(input).lines();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (node, value) = line.split_once(": ").unwrap();
        let node = parse_node(node)?;
        let value = value.parse::<u8>()?;
        ensure!(value < 2);
        assert!(values.insert(node, value == 1).is_none());
    }
    for line in lines {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        ensure!(words.len() == 5);
        ensure!(words[3] == "->");
        let node1 = parse_node(words[0])?;
        let node2 = parse_node(words[2])?;
        let node = parse_node(words[4])?;
        let gate = match words[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            x => bail!("bad gate {x}"),
        };
        assert!(gates.insert(node, (node1, gate, node2)).is_none());
    }
    let mut total = 0;
    for i in 0 .. {
        let node = [b'z', b'0' + (i / 10), b'0' + (i % 10)];
        if !gates.contains_key(&node) {
            break;
        }
        total |= (compute(&mut values, &gates, node) as usize) << i;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "52038112429798\n");
