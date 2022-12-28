use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn value(x: u8) -> usize {
    (match x {
        b'a' ..= b'z' => x - b'a' + 1,
        b'A' ..= b'Z' => x - b'A' + 27,
        _ => unreachable!(),
    }) as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()) {
        let n = line.len();
        assert_eq!(n % 2, 0);
        let n = n / 2;
        let a: HashSet<u8> = line[.. n].iter().cloned().collect();
        let b: HashSet<u8> = line[n ..].iter().cloned().collect();
        let i: Vec<u8> = a.intersection(&b).cloned().collect();
        assert_eq!(i.len(), 1);
        total += value(i[0]);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "7872\n");
