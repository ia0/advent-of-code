use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut input: Vec<_> = lines.next().unwrap()?.bytes().map(|x| (x - b'0') as usize).collect();
    assert!(lines.next().is_none());
    assert!(input.iter().all(|x| (0 ..= 9).contains(x)));
    let mut total = 0;
    let mut last = (input.len() - 1) / 2;
    let mut cur = 0;
    let mut pos = 0;
    while cur <= last {
        if 0 < input[2 * cur] {
            total += pos * cur;
            input[2 * cur] -= 1;
            pos += 1;
            continue;
        }
        if 0 < input[2 * cur + 1] {
            if 0 < input[2 * last] {
                total += pos * last;
                input[2 * last] -= 1;
                input[2 * cur + 1] -= 1;
                pos += 1;
            } else {
                last -= 1;
            }
            continue;
        }
        cur += 1;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "6283170117911\n");
