use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn conv(x: &str) -> [u8; 3] {
    x.as_bytes().try_into().unwrap()
}

fn total(
    edges: &HashMap<[u8; 3], Vec<[u8; 3]>>, memo: &mut HashMap<([u8; 3], u8), usize>, cur: [u8; 3],
    bit: u8,
) -> usize {
    if let Some(r) = memo.get(&(cur, bit)) {
        return *r;
    }
    let r = {
        if &cur == b"out" {
            (bit == 3) as usize
        } else {
            let mut bit = bit;
            bit |= (&cur == b"dac") as u8;
            bit |= 2 * (&cur == b"fft") as u8;
            edges[&cur].iter().map(|&next| total(edges, memo, next, bit)).sum()
        }
    };
    assert!(memo.insert((cur, bit), r).is_none());
    r
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut edges = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (src, dsts) = line.split_once(": ").unwrap();
        let dsts: Vec<_> = dsts.split_whitespace().map(conv).collect();
        assert!(edges.insert(conv(src), dsts).is_none());
    }
    let mut memo = HashMap::new();
    writeln!(output, "{}", total(&edges, &mut memo, *b"svr", 0))?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "502447498690860\n");
