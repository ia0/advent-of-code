use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut elves = Vec::new();
    let mut lines = BufReader::new(input).lines().map(|x| x.unwrap());
    let mut done = false;
    while !done {
        let mut elf: Vec<usize> = Vec::new();
        loop {
            let line = lines.next();
            done = line.is_none();
            let line = line.unwrap_or_default();
            if line.is_empty() {
                break;
            }
            elf.push(line.parse().unwrap());
        }
        elves.push(elf);
    }
    let mut top3 = BinaryHeap::new();
    for elf in elves {
        let calories: usize = elf.iter().sum();
        top3.push(Reverse(calories));
        if top3.len() > 3 {
            top3.pop();
        }
    }
    writeln!(output, "{}", top3.into_iter().map(|Reverse(x)| x).sum::<usize>())?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "206152\n");
