use std::collections::VecDeque;
use std::io::{Read, Write};

use adventofcode::{Coord, Frame};
use anyhow::Result;
use data_encoding::HEXLOWER;
use md5::{Digest, Md5};

#[derive(Default)]
struct Path(Vec<u8>);

impl Path {
    fn coord(&self) -> Coord {
        let mut pos = Coord::default();
        for &x in &self.0 {
            match x {
                b'U' => pos.y -= 1,
                b'D' => pos.y += 1,
                b'L' => pos.x -= 1,
                b'R' => pos.x += 1,
                _ => unreachable!(),
            }
        }
        pos
    }

    fn neighbors(&self, code: &[u8]) -> impl Iterator<Item = Path> + '_ {
        let mut pass = code.to_vec();
        pass.extend_from_slice(&self.0);
        let hash = HEXLOWER.encode(&Md5::digest(pass)).into_bytes();
        (0 .. 4).filter(move |&i| (b'b' ..= b'f').contains(&hash[i])).map(|i| {
            let x = match i {
                0 => b'U',
                1 => b'D',
                2 => b'L',
                3 => b'R',
                _ => unreachable!(),
            };
            let mut p = self.0.clone();
            p.push(x);
            Path(p)
        })
    }
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut code = Vec::new();
    input.read_to_end(&mut code)?;
    const FRAME: Frame = Frame { min: Coord { x: 0, y: 0 }, max: Coord { x: 3, y: 3 } };
    let mut todo = VecDeque::new();
    todo.push_back(Path::default());
    while let Some(path) = todo.pop_front() {
        for next in path.neighbors(&code) {
            let coord = next.coord();
            if coord == FRAME.max {
                writeln!(output, "{}", std::str::from_utf8(&next.0)?)?;
                return Ok(());
            }
            if FRAME.contains(coord) {
                todo.push_back(next);
            }
        }
    }
    unreachable!()
}

adventofcode::main!(solve("examples/17.txt") == "RRRLDRDUDD\n");
