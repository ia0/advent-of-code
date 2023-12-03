use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, ADJACENT_STAR};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut symbols = HashSet::new();
    let mut numbers = Vec::new();
    let mut digits = HashMap::new();
    for (y, line) in BufReader::new(input).lines().enumerate() {
        let y = y as i64;
        for (x, cell) in line?.bytes().enumerate() {
            let x = x as i64;
            let coord = Coord { x, y };
            match cell {
                b'0' ..= b'9' => {
                    if !digits.contains_key(&Coord { x: x - 1, y }) {
                        numbers.push(0);
                    }
                    let index = numbers.len() - 1;
                    numbers[index] = numbers[index] * 10 + (cell - b'0') as u64;
                    assert!(digits.insert(coord, index).is_none());
                }
                b'.' => (),
                _ => assert!(symbols.insert(coord)),
            }
        }
    }
    let mut parts = HashSet::new();
    for coord in symbols {
        for delta in ADJACENT_STAR {
            if let Some(&index) = digits.get(&(coord + delta)) {
                parts.insert(index);
            }
        }
    }
    let total: u64 = parts.into_iter().map(|i| numbers[i]).sum();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "550064\n");
