use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn trim(data: &str, open: char, close: char) -> &str {
    data.strip_prefix(open).unwrap().strip_suffix(close).unwrap()
}

fn parse(xs: &str) -> u16 {
    let mut r = 0;
    for x in trim(xs, '(', ')').split(',') {
        let b = x.parse::<u8>().unwrap();
        assert!(b <= 15);
        r |= 1 << b;
    }
    r
}

fn dist(target: u16, edges: &[u16]) -> usize {
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), 0));
    let mut visited = HashSet::new();
    while let Some((Reverse(dist), node)) = todo.pop() {
        if !visited.insert(node) {
            continue;
        }
        if node == target {
            return dist;
        }
        for &edge in edges {
            todo.push((Reverse(dist + 1), node ^ edge));
        }
    }
    unreachable!()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        let lights = trim(words.next().unwrap(), '[', ']');
        assert!(lights.len() <= 16);
        let lights: u16 = lights.bytes().enumerate().map(|(i, b)| ((b == b'#') as u16) << i).sum();
        let mut buttons: Vec<_> = words.collect();
        let _joltage = trim(buttons.pop().unwrap(), '{', '}');
        let buttons: Vec<_> = buttons.into_iter().map(parse).collect();
        total += dist(lights, &buttons);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "441\n");
