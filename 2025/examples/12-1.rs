use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut shapes = Vec::new();
    for i in 0 .. 6 {
        assert_eq!(lines.next().unwrap()?, format!("{i}:"));
        let mut count = 0;
        for _ in 0 .. 3 {
            let line = lines.next().unwrap()?;
            assert_eq!(line.len(), 3);
            assert!(line.bytes().all(|x| x == b'.' || x == b'#'));
            count += line.bytes().filter(|&x| x == b'#').count();
        }
        shapes.push(count);
        assert_eq!(lines.next().unwrap()?, "");
    }
    let mut total = 0;
    for line in lines {
        let line = line?;
        let (dims, counts) = line.split_once(": ").unwrap();
        let (width, height) = dims.split_once('x').unwrap();
        let width: usize = width.parse()?;
        let height: usize = height.parse()?;
        let counts: Vec<usize> = counts.split_whitespace().map(|x| x.parse().unwrap()).collect();
        assert_eq!(counts.len(), 6);
        let count: usize = (0 .. 6).map(|i| shapes[i] * counts[i]).sum();
        total += (count < width * height) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "587\n");
