use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut stacks = Vec::new();
    let mut lines = BufReader::new(input).lines();
    loop {
        let line = lines.next().unwrap()?.into_bytes();
        if line[1] == b'1' {
            break;
        }
        let mut i = 1;
        while i < line.len() {
            if line[i] != b' ' {
                let x = i / 4;
                if stacks.len() <= x {
                    stacks.resize(x + 1, Vec::new());
                }
                stacks[x].push(line[i]);
            }
            i += 4;
        }
    }
    assert!(lines.next().unwrap()?.is_empty());
    stacks.iter_mut().for_each(|x| x.reverse());
    for line in lines {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 6);
        assert_eq!(words[0], "move");
        assert_eq!(words[2], "from");
        assert_eq!(words[4], "to");
        let count: usize = words[1].parse()?;
        let from: usize = words[3].parse()?;
        let to: usize = words[5].parse()?;
        for _ in 0 .. count {
            let x = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(x);
        }
    }
    for stack in stacks {
        write!(output, "{}", *stack.last().unwrap() as char)?;
    }
    writeln!(output)?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "WSFTMRHPP\n");
