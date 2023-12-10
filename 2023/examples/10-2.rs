use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, Frame, ADJACENT_PLUS, EAST, NORTH, SOUTH, WEST};
use anyhow::{bail, ensure, Context, Error, Result};

struct Pipe {
    ends: [Coord; 2],
}

impl Pipe {
    fn pipe(&self, dir: Coord) -> Option<Coord> {
        for i in 0 .. 2 {
            if self.ends[i] == dir {
                return Some(self.ends[1 - i]);
            }
        }
        None
    }
}

enum Cell {
    Empty,
    Start,
    Pipe(Pipe),
}

impl TryFrom<u8> for Cell {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(match value {
            b'.' => Cell::Empty,
            b'S' => Cell::Start,
            b'|' => Cell::Pipe(Pipe { ends: [NORTH, SOUTH] }),
            b'-' => Cell::Pipe(Pipe { ends: [EAST, WEST] }),
            b'L' => Cell::Pipe(Pipe { ends: [NORTH, EAST] }),
            b'J' => Cell::Pipe(Pipe { ends: [NORTH, WEST] }),
            b'7' => Cell::Pipe(Pipe { ends: [SOUTH, WEST] }),
            b'F' => Cell::Pipe(Pipe { ends: [SOUTH, EAST] }),
            _ => bail!("unexpected cell {value:02x}"),
        })
    }
}

impl Cell {
    fn pipe(&self, dir: Coord) -> Option<Coord> {
        match self {
            Cell::Empty | Cell::Start => None,
            Cell::Pipe(x) => x.pipe(dir),
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut map = HashMap::new();
    for (y, row) in BufReader::new(input).lines().enumerate() {
        let y = y as i64;
        for (x, cell) in row?.bytes().enumerate() {
            let x = x as i64;
            let coord = Coord { x, y };
            ensure!(map.insert(coord, Cell::try_from(cell)?).is_none());
        }
    }
    let mut frame = Frame::new(map.keys().cloned()).context("empty map")?;
    frame.min += Coord { x: -1, y: -1 };
    frame.max += Coord { x: 1, y: 1 };
    let start =
        map.iter().find_map(|(x, y)| matches!(y, Cell::Start).then_some(*x)).context("no start")?;
    let ends = ADJACENT_PLUS
        .iter()
        .filter_map(|&x| map.get(&(start + x))?.pipe(-x).and(Some(x)))
        .collect::<Vec<_>>();
    ensure!(ends.len() == 2);
    let ends = [ends[0], ends[1]];
    map.insert(start, Cell::Pipe(Pipe { ends }));
    let mut dir = ends[0];
    let mut left = HashSet::new();
    let mut right = HashSet::new();
    let mut loop_ = HashSet::new();
    let mut cur = start;
    while loop_.insert(cur) {
        let new_left = dir.left();
        let new_right = dir.right();
        let old_dir = map[&cur].pipe(dir).unwrap();
        let old_left = old_dir.left();
        let old_right = old_dir.right();
        left.insert(cur + new_left);
        left.insert(cur + old_right);
        if new_left != old_right {
            left.insert(cur + new_left + old_right);
        }
        right.insert(cur + new_right);
        right.insert(cur + old_left);
        if new_right != old_left {
            right.insert(cur + new_right + old_left);
        }
        cur += dir;
        dir = map[&cur].pipe(-dir).context("broken pipe")?;
    }
    left.retain(|x| !loop_.contains(x));
    right.retain(|x| !loop_.contains(x));
    for visited in [&mut left, &mut right] {
        let mut todo = visited.iter().cloned().collect::<VecDeque<_>>();
        while let Some(pos) = todo.pop_front() {
            for dir in ADJACENT_PLUS {
                let pos = pos + dir;
                if !loop_.contains(&pos) && frame.contains(pos) && visited.insert(pos) {
                    todo.push_back(pos);
                }
            }
        }
    }
    let out = Coord { x: -1, y: -1 };
    let inner = match (left.contains(&out), right.contains(&out)) {
        (true, false) => right,
        (false, true) => left,
        x => bail!("outside conflict {x:?}"),
    };
    writeln!(output, "{}", inner.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "281\n");
