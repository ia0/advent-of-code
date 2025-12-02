use std::io::{Read, Write};

use anyhow::Result;

fn is_invalid(id: usize) -> bool {
    let id = format!("{id}");
    let n = id.len();
    for i in 2 ..= n {
        if n % i != 0 {
            continue;
        }
        if (1 .. i).all(|j| id[.. n / i] == id[j * n / i .. (j + 1) * n / i]) {
            return true;
        }
    }
    false
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

adventofcode::main!(solve("examples/02.txt") == "45814076230\n");
