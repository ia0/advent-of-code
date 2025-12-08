use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::UnionFind;
use anyhow::Result;

fn dist(a: [i64; 3], b: [i64; 3]) -> i64 {
    let mut t = 0;
    for i in 0 .. 3 {
        let d = a[i] - b[i];
        t += d * d;
    }
    t
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut nodes = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let node = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let node: [i64; 3] = node.try_into().unwrap();
        nodes.push(node);
    }
    let mut edges = Vec::new();
    for (i, &a) in nodes.iter().enumerate() {
        for &b in nodes.iter().take(i) {
            let d = dist(a, b);
            edges.push((-d, a, b));
        }
    }
    edges.sort();
    let n = nodes.len();
    let x = nodes[0];
    let mut uf = UnionFind::default();
    let mut result = 0;
    while uf.find(x).1 < n {
        let (_, a, b) = edges.pop().unwrap();
        result = a[0] * b[0];
        uf.union(a, b);
    }
    writeln!(output, "{result}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "51294528\n");
