use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{ADJACENT_PLUS, Coord};
use anyhow::Result;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (byte, x) in line?.bytes().zip(0 ..) {
            assert!(map.insert(Coord { x, y }, byte).is_none());
        }
    }
    let mut todo: HashSet<_> = map.keys().cloned().collect();
    let mut total = 0;
    while let Some(&pos) = todo.iter().next() {
        let mut perimeter = 0;
        let mut visited = HashSet::new();
        let mut next = vec![pos];
        let region = *map.get(&pos).unwrap();
        while let Some(pos) = next.pop() {
            if !visited.insert(pos) {
                continue;
            }
            assert!(todo.remove(&pos));
            for dir in ADJACENT_PLUS {
                let adj = pos + dir;
                if map.get(&adj) != Some(&region) {
                    perimeter += 1;
                    continue;
                }
                next.push(adj);
            }
        }
        total += visited.len() * perimeter;
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "1467094\n");
