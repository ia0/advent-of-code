use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{Read, Write};

use adventofcode::Coord;
use anyhow::Result;

fn open(d: usize, p: Coord) -> bool {
    if p.x < 0 || p.y < 0 {
        return false;
    }
    let x = p.x as usize;
    let y = p.y as usize;
    (x * x + 3 * x + 2 * x * y + y + y * y + d).count_ones() % 2 == 0
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut offset = String::new();
    input.read_to_string(&mut offset)?;
    let offset = offset.parse::<usize>()?;
    let start = Coord { x: 1, y: 1 };
    let end = Coord { x: 31, y: 39 };
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), start));
    let mut visited = HashSet::new();
    while let Some((Reverse(steps), cur)) = todo.pop() {
        if !visited.insert(cur) {
            continue;
        }
        if cur == end {
            writeln!(output, "{steps}")?;
            break;
        }
        let steps = steps + 1;
        for diff in adventofcode::ADJACENT_PLUS {
            let next = cur + diff;
            if open(offset, next) {
                todo.push((Reverse(steps), next));
            }
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/13.txt") == "82\n");
