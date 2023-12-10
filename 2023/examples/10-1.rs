use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, ADJACENT_PLUS, EAST, NORTH, SOUTH, WEST};
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
    let start =
        map.iter().find_map(|(x, y)| matches!(y, Cell::Start).then_some(*x)).context("no start")?;
    let mut todo = ADJACENT_PLUS
        .iter()
        .filter_map(|&x| map.get(&(start + x))?.pipe(-x).and(Some((0, start, x))))
        .collect::<VecDeque<_>>();
    let mut visited = HashMap::new();
    while let Some((dist, pos, dir)) = todo.pop_front() {
        let pos = pos + dir;
        if let Some(best) = visited.insert(pos, dist + 1) {
            writeln!(output, "{best}")?;
            break;
        }
        let dir = map[&pos].pipe(-dir).context("broken pipe")?;
        todo.push_back((dist + 1, pos, dir));
    }
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "7107\n");
