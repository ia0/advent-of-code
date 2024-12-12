use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, ADJACENT_PLUS};
use anyhow::Result;

fn sides(sides: &HashMap<(i64, i64), BTreeSet<i64>>) -> usize {
    let mut count = 0;
    for xs in sides.values() {
        let mut iter = xs.iter();
        let mut x = *iter.next().unwrap();
        for &y in iter {
            count += (y != x + 1) as usize;
            x = y;
        }
        count += 1;
    }
    count
}

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
        let mut hsides = HashMap::<(i64, i64), BTreeSet<i64>>::new();
        let mut vsides = HashMap::<(i64, i64), BTreeSet<i64>>::new();
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
                    if dir.x == 0 {
                        let y = std::cmp::min(pos.y, adj.y);
                        assert!(hsides.entry((y, pos.y)).or_default().insert(pos.x));
                    } else {
                        let x = std::cmp::min(pos.x, adj.x);
                        assert!(vsides.entry((x, pos.x)).or_default().insert(pos.y));
                    }
                    continue;
                }
                next.push(adj);
            }
        }
        total += visited.len() * (sides(&hsides) + sides(&vsides));
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "881182\n");
