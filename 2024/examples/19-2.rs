use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn find(cache: &mut HashMap<Vec<u8>, usize>, patterns: &HashSet<Vec<u8>>, xs: &[u8]) -> usize {
    if xs.is_empty() {
        return 1;
    }
    if let Some(&r) = cache.get(xs) {
        return r;
    }
    let mut total = 0;
    for pattern in patterns {
        if xs.get(.. pattern.len()).is_none_or(|x| x != pattern) {
            continue;
        }
        total += find(cache, patterns, &xs[pattern.len() ..]);
    }
    assert!(cache.insert(xs.to_vec(), total).is_none());
    total
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut patterns = HashSet::new();
    let mut cache = HashMap::new();
    for pattern in lines.next().unwrap()?.split(", ") {
        assert!(patterns.insert(pattern.bytes().collect()));
    }
    assert!(lines.next().unwrap()?.is_empty());
    let mut total = 0;
    for line in lines {
        total += find(&mut cache, &patterns, line?.as_bytes());
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "603191454138773\n");
