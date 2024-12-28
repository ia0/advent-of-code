use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{ADJACENT_PLUS, Coord};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            let pos = Coord { x, y };
            match byte {
                b'.' => (),
                b'#' => assert!(walls.insert(pos)),
                b'S' => assert!(start.replace(pos).is_none()),
                b'E' => assert!(end.replace(pos).is_none()),
                _ => unreachable!(),
            }
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), end));
    let mut visited = HashMap::new();
    while let Some((Reverse(dist), pos)) = todo.pop() {
        match visited.entry(pos) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(x) => drop(x.insert(dist)),
        }
        if pos == start {
            break;
        }
        for dir in ADJACENT_PLUS {
            let pos = pos + dir;
            if walls.contains(&pos) {
                continue;
            }
            todo.push((Reverse(dist + 1), pos));
        }
    }
    let mut total = 0;
    for &wall in &walls {
        for dir_start in ADJACENT_PLUS {
            let cheat_start = wall + dir_start;
            let Some(&dist_start) = visited.get(&cheat_start) else { continue };
            for dir_end in ADJACENT_PLUS {
                let cheat_end = wall + dir_end;
                let Some(&dist_end) = visited.get(&cheat_end) else { continue };
                if dist_start + 100 + 2 <= dist_end {
                    total += 1;
                }
            }
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/20.txt") == "1343\n");
