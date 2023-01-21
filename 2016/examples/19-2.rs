use std::collections::VecDeque;
use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut count = String::new();
    input.read_to_string(&mut count)?;
    let count = count.parse::<usize>()?;
    let mut left = (0 .. count / 2).collect::<VecDeque<_>>();
    let mut right = (count / 2 .. count).collect::<VecDeque<_>>();
    while !left.is_empty() {
        assert!(left.len() == right.len() || left.len() + 1 == right.len());
        right.pop_front();
        right.push_back(left.pop_front().unwrap());
        if left.len() + 1 < right.len() {
            left.push_back(right.pop_front().unwrap());
        }
    }
    writeln!(output, "{}", right[0] + 1)?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "1420064\n");
