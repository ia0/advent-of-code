use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

const STEPS: [Coord; 5] = [
    Coord { x: -1, y: 0 },
    Coord { x: 0, y: -1 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: 1 },
    Coord { x: 0, y: 0 },
];

#[derive(Default)]
struct Problem {
    right: Vec<Vec<bool>>,
    left: Vec<Vec<bool>>,
    down: Vec<Vec<bool>>,
    up: Vec<Vec<bool>>,
}

impl Problem {
    fn lim_x(&self) -> i64 {
        self.down.len() as i64
    }

    fn lim_y(&self) -> i64 {
        self.right.len() as i64
    }

    fn start(&self) -> Coord {
        Coord { x: 0, y: -1 }
    }

    fn end(&self) -> Coord {
        Coord { x: self.lim_x() - 1, y: self.lim_y() }
    }

    fn is_step(&self, time: i64, pos: Coord) -> bool {
        if pos == self.start() || pos == self.end() {
            return true;
        }
        (0 <= pos.x && pos.x < self.lim_x() && 0 <= pos.y && pos.y < self.lim_y())
            && !(self.right[pos.y as usize][(pos.x - time).rem_euclid(self.lim_x()) as usize]
                || self.left[pos.y as usize][(pos.x + time).rem_euclid(self.lim_x()) as usize]
                || self.down[pos.x as usize][(pos.y - time).rem_euclid(self.lim_y()) as usize]
                || self.up[pos.x as usize][(pos.y + time).rem_euclid(self.lim_y()) as usize])
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut problem = Problem::default();
    for line in BufReader::new(input).lines() {
        let line = line?.into_bytes();
        if line[1] == b'#' || line[line.len() - 2] == b'#' {
            continue;
        }
        let y = problem.right.len();
        problem.right.push(Vec::new());
        problem.left.push(Vec::new());
        for &b in &line[1 .. line.len() - 1] {
            let x = problem.right[y].len();
            if y == 0 {
                problem.down.push(Vec::new());
                problem.up.push(Vec::new());
            }
            problem.right[y].push(b == b'>');
            problem.left[y].push(b == b'<');
            problem.down[x].push(b == b'v');
            problem.up[x].push(b == b'^');
        }
    }
    let mut todo = BinaryHeap::new();
    let mut visited = HashSet::new();
    todo.push((Reverse(0), problem.start()));
    while let Some((Reverse(time), cur)) = todo.pop() {
        if !visited.insert((time, cur)) {
            continue;
        }
        if cur == problem.end() {
            writeln!(output, "{time}")?;
            break;
        }
        for &step in &STEPS {
            let next = cur + step;
            if problem.is_step(time + 1, next) {
                todo.push((Reverse(time + 1), next));
            }
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/24.txt") == "255\n");
