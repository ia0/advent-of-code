use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Frame;
use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashSet::new();
    for (y, line) in (1 ..).zip(BufReader::new(input).lines()) {
        let line = line?;
        for (x, b) in (1 ..).zip(line.bytes()) {
            let coord = Coord { x, y };
            match b {
                b'.' => (),
                b'#' => assert!(map.insert(coord)),
                _ => unreachable!(),
            }
        }
    }
    let mut dests = [
        [Coord { x: -1, y: -1 }, Coord { x: 0, y: -1 }, Coord { x: 1, y: -1 }],
        [Coord { x: -1, y: 1 }, Coord { x: 0, y: 1 }, Coord { x: 1, y: 1 }],
        [Coord { x: -1, y: -1 }, Coord { x: -1, y: 0 }, Coord { x: -1, y: 1 }],
        [Coord { x: 1, y: -1 }, Coord { x: 1, y: 0 }, Coord { x: 1, y: 1 }],
    ];
    for _ in 0 .. 10 {
        let mut from = HashMap::new();
        for &src in map.iter() {
            if adventofcode::ADJACENT_STAR.iter().all(|&delta| !map.contains(&(src + delta))) {
                continue;
            }
            for &dest in dests.iter() {
                if dest.iter().all(|&delta| !map.contains(&(src + delta))) {
                    let dst = src + dest[1];
                    match from.entry(dst) {
                        Entry::Occupied(mut x) => drop(x.insert(None)),
                        Entry::Vacant(x) => drop(x.insert(Some(src))),
                    }
                    break;
                }
            }
        }
        for (dst, src) in from {
            if let Some(src) = src {
                assert!(map.insert(dst));
                assert!(map.remove(&src));
            }
        }
        dests.rotate_left(1);
    }
    let frame = Frame::new(map.iter().cloned()).unwrap();
    let mut count = 0;
    for x in frame.min.x ..= frame.max.x {
        for y in frame.min.y ..= frame.max.y {
            let coord = Coord { x, y };
            count += !map.contains(&coord) as usize;
        }
    }
    writeln!(output, "{count}")?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "4045\n");
