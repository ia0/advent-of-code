use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut lines = BufReader::new(input).lines();
    loop {
        let init = lines.next().unwrap()?.into_bytes();
        assert_eq!(init.len(), 5);
        let mut lens = [0u8; 5];
        for _ in 0 .. 6 {
            let line = lines.next().unwrap()?.into_bytes();
            assert_eq!(line.len(), 5);
            for i in 0 .. 5 {
                lens[i] += (line[i] == init[i]) as u8;
            }
        }
        (if init[0] == b'#' { &mut locks } else { &mut keys }).push(lens);
        match lines.next() {
            Some(x) => assert!(x?.is_empty()),
            None => break,
        }
    }
    let mut total = 0;
    for lock in &locks {
        for key in &keys {
            total += lock.iter().zip(key.iter()).all(|(a, b)| a <= b) as usize;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/25.txt") == "2770\n");
