use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{UnionFind, topn};
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
    let mut uf = UnionFind::default();
    for &(_, a, b) in topn(1000, edges.iter()) {
        uf.union(a, b);
    }
    let total: usize = topn(3, uf.roots().map(|(_, s)| s)).product();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "80446\n");
