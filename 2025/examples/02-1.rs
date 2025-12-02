use std::io::{Read, Write};

use anyhow::Result;

fn is_invalid(id: usize) -> bool {
    let id = format!("{id}");
    let n = id.len();
    if n % 2 == 1 {
        return false;
    }
    id[.. n / 2] == id[n / 2 ..]
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut ranges = String::new();
    input.read_to_string(&mut ranges).unwrap();
    let mut total = 0;
    for range in ranges.strip_suffix("\n").unwrap().split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let first: usize = first.parse().unwrap();
        let last: usize = last.parse().unwrap();
        for id in first ..= last {
            if is_invalid(id) {
                total += id;
            }
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "35367539282\n");
