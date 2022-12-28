use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut grid = HashMap::new();
    let mut end = None;
    for (y, line) in BufReader::new(input).lines().enumerate() {
        let y = y as i64;
        for (x, mut byte) in line?.bytes().enumerate() {
            let x = x as i64;
            match byte {
                b'S' => byte = b'a',
                b'E' => {
                    byte = b'z';
                    assert!(end.replace(Coord { x, y }).is_none());
                }
                _ => (),
            }
            assert!(grid.insert(Coord { x, y }, byte).is_none());
        }
    }
    let mut todo = BinaryHeap::new();
    let mut visited = HashSet::new();
    todo.push((Reverse(0), end.unwrap()));
    while let Some((Reverse(dist), cur)) = todo.pop() {
        if grid[&cur] == b'a' {
            writeln!(output, "{dist}")?;
            break;
        }
        if !visited.insert(cur) {
            continue;
        }
        let cur_height = grid[&cur];
        for step in adventofcode::ADJACENT_PLUS {
            let next = cur + step;
            if let Some(&next_height) = grid.get(&next) {
                if cur_height - 1 <= next_height {
                    todo.push((Reverse(dist + 1), next));
                }
            }
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/12.txt") == "418\n");
