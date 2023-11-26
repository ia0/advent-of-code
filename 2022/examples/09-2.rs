use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

#[derive(Default)]
struct State {
    knot: [Coord; 10],
    visited: HashSet<Coord>,
}

impl State {
    fn step(&mut self, dir: u8) {
        match dir {
            b'U' => self.knot[0].y -= 1,
            b'D' => self.knot[0].y += 1,
            b'L' => self.knot[0].x -= 1,
            b'R' => self.knot[0].x += 1,
            _ => unreachable!(),
        }
        for i in 1 .. 10 {
            self.adjust(i);
        }
        self.visited.insert(self.knot[9]);
    }

    fn adjust(&mut self, i: usize) {
        let diff = self.knot[i] - self.knot[i - 1];
        match (diff.x, diff.y) {
            (-1 ..= 1, -1 ..= 1) => (),
            (-2, y @ (-1 ..= 1)) => {
                self.knot[i].x += 1;
                self.knot[i].y -= y;
            }
            (2, y @ (-1 ..= 1)) => {
                self.knot[i].x -= 1;
                self.knot[i].y -= y;
            }
            (x @ (-1 ..= 1), -2) => {
                self.knot[i].x -= x;
                self.knot[i].y += 1;
            }
            (x @ (-1 ..= 1), 2) => {
                self.knot[i].x -= x;
                self.knot[i].y -= 1;
            }
            (x @ (-2 | 2), y @ (-2 | 2)) => {
                self.knot[i].x -= x / 2;
                self.knot[i].y -= y / 2;
            }
            _ => unreachable!(),
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut state = State::default();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let words: Vec<_> = line.split_whitespace().collect();
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].len(), 1);
        let dir = words[0].as_bytes()[0];
        let len: usize = words[1].parse().unwrap();
        for _ in 0 .. len {
            state.step(dir);
        }
    }
    writeln!(output, "{}", state.visited.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/09.txt") == "2653\n");
