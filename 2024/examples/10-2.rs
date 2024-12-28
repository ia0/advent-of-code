use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{ADJACENT_PLUS, Coord};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut trails: [_; 10] = std::array::from_fn(|_| HashMap::<Coord, u64>::new());
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            assert!(byte.is_ascii_digit());
            let height = (byte - b'0') as usize;
            trails[height].insert(pos, (height == 9) as u64);
        }
    }
    for height in (0 .. 9).rev() {
        let (head, tail) = trails.split_at_mut(height + 1);
        let head = head.last_mut().unwrap();
        let tail = tail.first().unwrap();
        for (&pos, map) in head.iter_mut() {
            for dir in ADJACENT_PLUS {
                let Some(count) = tail.get(&(pos + dir)) else { continue };
                *map += count;
            }
        }
    }
    let mut total = 0;
    for map in trails[0].values() {
        total += map;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "1372\n");
