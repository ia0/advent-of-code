use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cell(u8);

impl Cell {
    fn empty() -> Cell {
        Cell(0)
    }

    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn new(x: u8) -> Cell {
        assert!(b'A' <= x && x <= b'D');
        Cell(x - b'A' + 1)
    }

    fn cost(self) -> usize {
        assert!(!self.is_empty());
        (1 .. self.0).map(|_| 10).product()
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.0 {
            0 => write!(f, "."),
            x => write!(f, "{}", (b'A' + x - 1) as char),
        }
    }
}

// 01.2.3.4.56
//   7 8 9 a
//   b c d e
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State([Cell; 15]);

const PATHS: [&[(usize, &[usize], usize)]; 15] = [
    &[
        // 0
        (7, &[1], 1),
        (11, &[1, 7], 1),
        (8, &[1, 2], 2),
        (12, &[1, 2, 8], 2),
        (9, &[1, 2, 3], 3),
        (13, &[1, 2, 3, 9], 3),
        (10, &[1, 2, 3, 4], 4),
        (14, &[1, 2, 3, 4, 10], 4),
    ],
    &[
        // 1
        (7, &[], 1),
        (11, &[7], 1),
        (8, &[2], 2),
        (12, &[2, 8], 2),
        (9, &[2, 3], 3),
        (13, &[2, 3, 9], 3),
        (10, &[2, 3, 4], 4),
        (14, &[2, 3, 4, 10], 4),
    ],
    &[
        // 2
        (7, &[], 1),
        (11, &[7], 1),
        (8, &[], 1),
        (12, &[8], 1),
        (9, &[3], 2),
        (13, &[3, 9], 2),
        (10, &[3, 4], 3),
        (14, &[3, 4, 10], 3),
    ],
    &[
        // 3
        (7, &[2], 2),
        (11, &[2, 7], 2),
        (8, &[], 1),
        (12, &[8], 1),
        (9, &[], 1),
        (13, &[9], 1),
        (10, &[4], 2),
        (14, &[4, 10], 2),
    ],
    &[
        // 4
        (7, &[3, 2], 3),
        (11, &[3, 2, 7], 3),
        (8, &[3], 2),
        (12, &[3, 8], 2),
        (9, &[], 1),
        (13, &[9], 1),
        (10, &[], 1),
        (14, &[10], 1),
    ],
    &[
        // 5
        (7, &[4, 3, 2], 4),
        (11, &[4, 3, 2, 7], 4),
        (8, &[4, 3], 3),
        (12, &[4, 3, 8], 3),
        (9, &[4], 2),
        (13, &[4, 9], 2),
        (10, &[], 1),
        (14, &[10], 1),
    ],
    &[
        // 6
        (7, &[5, 4, 3, 2], 4),
        (11, &[5, 4, 3, 2, 7], 4),
        (8, &[5, 4, 3], 3),
        (12, &[5, 4, 3, 8], 3),
        (9, &[5, 4], 2),
        (13, &[5, 4, 9], 2),
        (10, &[5], 1),
        (14, &[5, 10], 1),
    ],
    &[
        // 7
        (0, &[1], 1),
        (1, &[], 1),
        (2, &[], 1),
        (3, &[2], 2),
        (4, &[2, 3], 3),
        (5, &[2, 3, 4], 4),
        (6, &[2, 3, 4, 5], 4),
    ],
    &[
        // 8
        (0, &[2, 1], 2),
        (1, &[2], 2),
        (2, &[], 1),
        (3, &[], 1),
        (4, &[3], 2),
        (5, &[3, 4], 3),
        (6, &[3, 4, 5], 3),
    ],
    &[
        // 9
        (0, &[3, 2, 1], 3),
        (1, &[3, 2], 3),
        (2, &[3], 2),
        (3, &[], 1),
        (4, &[], 1),
        (5, &[4], 2),
        (6, &[4, 5], 2),
    ],
    &[
        // 10
        (0, &[4, 3, 2, 1], 4),
        (1, &[4, 3, 2], 4),
        (2, &[4, 3], 3),
        (3, &[4], 2),
        (4, &[], 1),
        (5, &[], 1),
        (6, &[5], 1),
    ],
    &[
        // 11
        (0, &[7, 1], 1),
        (1, &[7], 1),
        (2, &[7], 1),
        (3, &[7, 2], 2),
        (4, &[7, 2, 3], 3),
        (5, &[7, 2, 3, 4], 4),
        (6, &[7, 2, 3, 4, 5], 4),
    ],
    &[
        // 12
        (0, &[8, 2, 1], 2),
        (1, &[8, 2], 2),
        (2, &[8], 1),
        (3, &[8], 1),
        (4, &[8, 3], 2),
        (5, &[8, 3, 4], 3),
        (6, &[8, 3, 4, 5], 3),
    ],
    &[
        // 13
        (0, &[9, 3, 2, 1], 3),
        (1, &[9, 3, 2], 3),
        (2, &[9, 3], 2),
        (3, &[9], 1),
        (4, &[9], 1),
        (5, &[9, 4], 2),
        (6, &[9, 4, 5], 2),
    ],
    &[
        // 14
        (0, &[10, 4, 3, 2, 1], 4),
        (1, &[10, 4, 3, 2], 4),
        (2, &[10, 4, 3], 3),
        (3, &[10, 4], 2),
        (4, &[10], 1),
        (5, &[10], 1),
        (6, &[10, 5], 1),
    ],
];

impl State {
    fn is_done(&self) -> bool {
        (0 .. 4).all(|j| (0 .. 2).all(|i| self.0[7 + 4 * i + j].0 as usize == j + 1))
    }

    fn edges(&self) -> Vec<(usize, State)> {
        let mut edges = Vec::new();
        for i in 0 .. 15 {
            if self.0[i].is_empty() {
                continue;
            }
            let cost = self.0[i].cost();
            for &(dest, path, extra) in PATHS[i] {
                if !self.0[dest].is_empty() || !path.iter().all(|&x| self.0[x].is_empty()) {
                    continue;
                }
                if i < 7 {
                    let t = ((dest - 7) % 4) as u8;
                    if t != self.0[i].0 - 1 {
                        continue;
                    }
                    let other = (7 + (t ^ 4)) as usize;
                    if !self.0[other].is_empty() && self.0[other].0 != self.0[i].0 {
                        continue;
                    }
                }
                let mut next = *self;
                next.0.swap(dest, i);
                edges.push((cost * (1 + path.len() + extra), next));
            }
        }
        edges
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = |i| self.0[i];
        writeln!(f, "{}{} {} {} {} {}{}", p(0), p(1), p(2), p(3), p(4), p(5), p(6))?;
        writeln!(f, "  {} {} {} {}", p(7), p(8), p(9), p(10))?;
        write!(f, "  {} {} {} {}", p(11), p(12), p(13), p(14))
    }
}

fn main() {
    let input = File::open("examples/23.txt").unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|x| x.unwrap()).collect();
    let mut state = [Cell::empty(); 15];
    for i in 0 .. 2 {
        for j in 0 .. 4 {
            state[7 + 4 * i + j] = Cell::new(lines[2 + i].as_bytes()[3 + 2 * j]);
        }
    }
    let mut done = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push((Reverse(0), State(state)));
    while let Some((Reverse(cost), state)) = todo.pop() {
        if state.is_done() {
            println!("{}", cost);
            return;
        }
        if !done.insert(state) {
            continue;
        }
        for (delta, next) in state.edges() {
            todo.push((Reverse(cost + delta), next));
        }
    }
    unreachable!()
}
