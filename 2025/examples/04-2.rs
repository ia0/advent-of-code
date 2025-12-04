use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{ADJACENT_STAR, Coord};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut rolls = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            match byte {
                b'.' => continue,
                b'@' => (),
                _ => unreachable!(),
            }
            let pos = Coord { x, y };
            assert!(rolls.insert(pos));
        }
    }
    let mut total = 0;
    loop {
        let mut todo = HashSet::new();
        for &pos in &rolls {
            let has_roll = |&dir: &Coord| rolls.contains(&(pos + dir));
            if ADJACENT_STAR.into_iter().filter(has_roll).count() < 4 {
                assert!(todo.insert(pos));
            }
        }
        if todo.is_empty() {
            break;
        }
        total += todo.len();
        for pos in todo {
            assert!(rolls.remove(&pos));
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "9120\n");
