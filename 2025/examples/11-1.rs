use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn conv(x: &str) -> [u8; 3] {
    x.as_bytes().try_into().unwrap()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut edges = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (src, dsts) = line.split_once(": ").unwrap();
        let dsts: Vec<_> = dsts.split_whitespace().map(conv).collect();
        assert!(edges.insert(conv(src), dsts).is_none());
    }
    let mut total = 0;
    let mut frames = vec![(b"you", HashSet::new())];
    while let Some((cur, mut visited)) = frames.pop() {
        if cur == b"out" {
            total += 1;
            continue;
        }
        if !visited.insert(cur) {
            continue;
        }
        for next in &edges[cur] {
            frames.push((next, visited.clone()));
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "555\n");
