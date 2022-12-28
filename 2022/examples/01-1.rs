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
    writeln!(output, "{}", elves.iter().map(|x| x.iter().sum::<usize>()).max().unwrap())?;
    Ok(())
}

adventofcode::main!(solve("examples/01.txt") == "69528\n");
