use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn mix_prune(secret: usize, new: usize) -> usize {
    (secret ^ new) % 16777216
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut cache = HashMap::<Vec<i8>, HashMap<usize, usize>>::new();
    for (buyer, line) in BufReader::new(input).lines().enumerate() {
        let mut secret = line?.parse()?;
        let mut prev = (secret % 10) as i8;
        let mut sequence = Vec::new();
        for _ in 0 .. 2000 {
            secret = mix_prune(secret, secret * 64);
            secret = mix_prune(secret, secret / 32);
            secret = mix_prune(secret, secret * 2048);
            let new = (secret % 10) as i8;
            sequence.push(new - prev);
            if 4 < sequence.len() {
                sequence.remove(0);
            }
            if sequence.len() == 4 {
                match cache.entry(sequence.clone()).or_default().entry(buyer) {
                    Entry::Occupied(_) => (),
                    Entry::Vacant(x) => drop(x.insert(new as usize)),
                }
            }
            prev = new;
        }
    }
    let total = cache.values().map(|x| x.values().sum::<usize>()).max().unwrap();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "1968\n");
