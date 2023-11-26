use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Coord = adventofcode::Coord<i64>;

#[derive(Default)]
struct State {
    head: Coord,
    tail: Coord,
    visited: HashSet<Coord>,
}

impl State {
    fn step(&mut self, dir: u8) {
        match dir {
            b'U' => self.head.y -= 1,
            b'D' => self.head.y += 1,
            b'L' => self.head.x -= 1,
            b'R' => self.head.x += 1,
            _ => unreachable!(),
        }
        self.adjust();
    }

    fn adjust(&mut self) {
        let diff = self.tail - self.head;
        match (diff.x, diff.y) {
            (-1 ..= 1, -1 ..= 1) => (),
            (-2, y @ (-1 ..= 1)) => {
                self.tail.x += 1;
                self.tail.y -= y;
            }
            (2, y @ (-1 ..= 1)) => {
                self.tail.x -= 1;
                self.tail.y -= y;
            }
            (x @ (-1 ..= 1), -2) => {
                self.tail.x -= x;
                self.tail.y += 1;
            }
            (x @ (-1 ..= 1), 2) => {
                self.tail.x -= x;
                self.tail.y -= 1;
            }
            _ => unreachable!(),
        }
        self.visited.insert(self.tail);
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

adventofcode::main!(solve("examples/09.txt") == "6563\n");
