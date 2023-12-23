use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS};
use anyhow::{ensure, Context, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut walls = HashSet::new();
    for (line, y) in BufReader::new(input).lines().zip(0 ..) {
        for (cell, x) in line?.bytes().zip(0 ..) {
            let coord = Coord { x, y };
            ensure!(b"#.<>^v".contains(&cell));
            if cell == b'#' {
                walls.insert(coord);
            }
        }
    }
    let frame = Frame::new(walls.iter().cloned()).context("no walls")?;
    let start = Coord { x: frame.min.x + 1, y: frame.min.y };
    let end = Coord { x: frame.max.x - 1, y: frame.max.y };
    let mut edges = HashMap::<Coord, HashMap<Coord, usize>>::new();
    for a in frame.iter().filter(|x| !walls.contains(x)) {
        for dir in ADJACENT_PLUS {
            let b = a + dir;
            if frame.contains(b) && !walls.contains(&b) {
                assert!(edges.entry(a).or_default().insert(b, 1).is_none());
            }
        }
    }
    let mut nodes = edges.keys().cloned().collect::<HashSet<_>>();
    let mut done = false;
    while !done {
        done = true;
        nodes.retain(|b| match edges[b].len() {
            _ if [start, end].contains(b) => true,
            0 => unreachable!(),
            1 => {
                let a = edges.remove(b).unwrap().into_iter().collect::<Vec<_>>();
                edges.get_mut(&a[0].0).unwrap().remove(b).unwrap();
                done = false;
                false
            }
            2 => {
                let ac = edges.remove(b).unwrap().into_iter().collect::<Vec<_>>();
                assert_eq!(edges.get_mut(&ac[0].0).unwrap().remove(b).unwrap(), ac[0].1);
                assert_eq!(edges.get_mut(&ac[1].0).unwrap().remove(b).unwrap(), ac[1].1);
                let d = ac[0].1 + ac[1].1;
                edges.get_mut(&ac[0].0).unwrap().insert(ac[1].0, d);
                edges.get_mut(&ac[1].0).unwrap().insert(ac[0].0, d);
                done = false;
                false
            }
            _ => true,
        });
    }
    assert!(edges.keys().cloned().collect::<HashSet<_>>() == nodes);
    assert!(nodes.contains(&start) && nodes.contains(&end));
    let mut best = 0;
    let mut todo = vec![(HashSet::<Coord>::new(), 0, start)];
    while let Some((mut visited, dist, pos)) = todo.pop() {
        let mut next = Vec::new();
        for (&pos, &delta) in &edges[&pos] {
            let dist = dist + delta;
            if pos == end {
                best = std::cmp::max(best, dist);
            } else if !visited.contains(&pos) {
                next.push((dist, pos));
            }
        }
        let mut next = next.into_iter();
        let last = match next.next() {
            Some(x) => x,
            None => continue,
        };
        for (dist, pos) in next {
            let mut visited = visited.clone();
            assert!(visited.insert(pos));
            todo.push((visited, dist, pos));
        }
        let (dist, pos) = last;
        assert!(visited.insert(pos));
        todo.push((visited, dist, pos));
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/23.txt") == "6514\n");
