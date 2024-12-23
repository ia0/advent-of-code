use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn conv(x: &str) -> [u8; 2] {
    x.as_bytes().try_into().unwrap()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut graph = HashMap::<[u8; 2], HashSet<[u8; 2]>>::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (a, b) = line.split_once("-").unwrap();
        let a = conv(a);
        let b = conv(b);
        assert!(a != b);
        assert!(graph.entry(a).or_default().insert(b));
        assert!(graph.entry(b).or_default().insert(a));
    }
    let mut total = HashSet::new();
    for (a, bs) in &graph {
        if a[0] != b't' {
            continue;
        }
        for b in bs {
            for c in graph.get(b).unwrap() {
                if graph.get(c).unwrap().contains(a) {
                    let mut x = [a, b, c];
                    x.sort();
                    total.insert(x);
                }
            }
        }
    }
    writeln!(output, "{}", total.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "1218\n");
