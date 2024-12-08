use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut antennas = HashMap::<u8, HashSet<Coord>>::new();
    let mut frame = Frame::default();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        frame.max.y = y;
        for (byte, x) in line?.bytes().zip(0 ..) {
            frame.max.x = x;
            if byte == b'.' {
                continue;
            }
            let pos = Coord { x, y };
            assert!(antennas.entry(byte).or_default().insert(pos));
        }
    }
    let mut antinodes = HashSet::new();
    for group in antennas.values() {
        for &a in group.iter() {
            for &b in group.iter() {
                if a == b {
                    continue;
                }
                let c = b * 2 - a;
                if frame.contains(c) {
                    antinodes.insert(c);
                }
            }
        }
    }
    let total = antinodes.len();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/08.txt") == "289\n");
