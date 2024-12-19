use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

#[derive(Debug, Default)]
struct Trie(HashMap<Option<u8>, Trie>);

fn find(trie: &Trie, cur: &Trie, xs: &[u8]) -> bool {
    if cur.0.contains_key(&None) && find(trie, trie, xs) {
        return true;
    }
    let Some((&x, xs)) = xs.split_first() else { return cur.0.contains_key(&None) };
    match cur.0.get(&Some(x)) {
        None => false,
        Some(cur) => find(trie, cur, xs),
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut trie = Trie::default();
    for pattern in lines.next().unwrap()?.split(", ") {
        let mut cur = &mut trie.0;
        for byte in pattern.bytes() {
            cur = &mut cur.entry(Some(byte)).or_default().0;
        }
        cur.entry(None).or_default();
    }
    assert!(lines.next().unwrap()?.is_empty());
    let mut total = 0;
    for line in lines {
        total += find(&trie, &trie, line?.as_bytes()) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "216\n");
