use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cell(u8);

impl Cell {
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    // 01.2.3.4.56
    hall: [Cell; 7],
    // [column][row]
    room: [[Cell; 4]; 4],
}

impl State {
    fn is_done(&self) -> bool {
        (0 .. 4).all(|j| (0 .. 4).all(|i| self.room[i][j].0 as usize == 1 + i))
    }

    fn edges(&self) -> Vec<(usize, State)> {
        let mut edges = Vec::new();
        for i in 0 .. 7 {
            if self.hall[i].is_empty() {
                continue;
            }
            let d = (self.hall[i].0 - 1) as usize;
            // Room is valid.
            if !self.room[d].iter().all(|x| x.is_empty() || x.0 == 1 + d as u8) {
                continue;
            }
            // Hall is free.
            let cost = match self.path(d, i) {
                None => continue,
                Some(x) => x,
            };
            // Find room position.
            let j = self.room[d as usize].iter().position(|x| !x.is_empty()).unwrap_or(4) - 1;
            let cost = self.hall[i].cost() * (j + cost);
            let mut next = *self;
            std::mem::swap(&mut next.hall[i], &mut next.room[d][j]);
            edges.push((cost, next));
        }
        for i in 0 .. 4 {
            let j = match self.room[i].iter().position(|x| !x.is_empty()) {
                None => continue,
                Some(x) => x,
            };
            for d in 0 .. 7 {
                if !self.hall[d].is_empty() {
                    continue;
                }
                // Hall is free
                let cost = match self.path(i, d) {
                    None => continue,
                    Some(x) => x,
                };
                let cost = self.room[i][j].cost() * (j + cost);
                let mut next = *self;
                std::mem::swap(&mut next.hall[d], &mut next.room[i][j]);
                edges.push((cost, next));
            }
        }
        edges
    }

    fn path(&self, room: usize, hall: usize) -> Option<usize> {
        let mut path = if hall <= room + 1 { hall + 1 ..= room + 1 } else { room + 2 ..= hall - 1 };
        if path.all(|x| self.hall[x].is_empty()) {
            if hall <= room + 1 {
                Some(2 * (room + 2 - hall) - (hall == 0) as usize)
            } else {
                Some(2 * (hall - 1 - room) - (hall == 6) as usize)
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let p = |i| self.hall[i];
        writeln!(f, "{}{} {} {} {} {}{}", p(0), p(1), p(2), p(3), p(4), p(5), p(6))?;
        for i in 0 .. 4 {
            write!(f, " ")?;
            for j in 0 .. 4 {
                write!(f, " {}", self.room[j][i])?;
            }
            if i < 3 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let input = File::open("examples/23.txt").unwrap();
    let lines: Vec<String> = BufReader::new(input).lines().map(|x| x.unwrap()).collect();
    let mut state = State::default();
    for i in 0 .. 2 {
        for j in 0 .. 4 {
            state.room[j][3 * i] = Cell::new(lines[2 + i].as_bytes()[3 + 2 * j]);
        }
    }
    for (i, &x) in [4, 3, 2, 1].iter().enumerate() {
        state.room[i][1] = Cell(x);
    }
    for (i, &x) in [4, 2, 1, 3].iter().enumerate() {
        state.room[i][2] = Cell(x);
    }
    let mut done = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse((0, state)));
    while let Some(Reverse((cost, state))) = todo.pop() {
        if state.is_done() {
            println!("{}", cost);
            return;
        }
        if !done.insert(state) {
            continue;
        }
        for (delta, next) in state.edges() {
            todo.push(Reverse((cost + delta, next)));
        }
    }
    unreachable!()
}
