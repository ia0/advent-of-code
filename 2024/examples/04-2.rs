use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn read(word: &[u8], map: &HashMap<Coord, u8>, pos: Coord, dir: Coord) -> bool {
    let Some((next, word)) = word.split_first() else { return true };
    if map.get(&pos) != Some(next) {
        return false;
    }
    read(word, map, pos + dir, dir)
}

fn read_mas(map: &HashMap<Coord, u8>, pos: Coord, dir: Coord) -> bool {
    read(b"MAS", map, pos - dir, dir) || read(b"SAM", map, pos - dir, dir)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    let mut candidates = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            assert!(map.insert(pos, byte).is_none());
            if byte == b'A' {
                assert!(candidates.insert(pos));
            }
        }
    }
    let mut total = 0;
    for pos in candidates {
        total += (read_mas(&map, pos, Coord { x: 1, y: 1 })
            && read_mas(&map, pos, Coord { x: 1, y: -1 })) as usize;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "2034\n");
