use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, ADJACENT_STAR};
use anyhow::Result;

fn read(word: &[u8], map: &HashMap<Coord, u8>, pos: Coord, dir: Coord) -> bool {
    let Some((next, word)) = word.split_first() else { return true };
    if map.get(&pos) != Some(next) {
        return false;
    }
    read(word, map, pos + dir, dir)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    let mut candidates = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            assert!(map.insert(pos, byte).is_none());
            if byte == b'X' {
                assert!(candidates.insert(pos));
            }
        }
    }
    let mut total = 0;
    for pos in candidates {
        for dir in ADJACENT_STAR {
            total += read(b"XMAS", &map, pos, dir) as usize;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "2662\n");
