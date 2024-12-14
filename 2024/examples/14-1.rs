use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::Result;

#[derive(Debug)]
struct Robot {
    pos: Coord,
    vel: Coord,
}

impl Robot {
    fn parse(x: &str) -> Result<Robot> {
        let (pos, vel) = x.split_once(" ").unwrap();
        Ok(Robot { pos: Coord::parse(&pos[2 ..], ",")?, vel: Coord::parse(&vel[2 ..], ",")? })
    }

    fn advance(&mut self, frame: Coord, count: i64) {
        self.pos += self.vel * count;
        self.pos %= frame;
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut robots = Vec::new();
    for line in BufReader::new(input).lines() {
        robots.push(Robot::parse(&line?)?);
    }
    let frame = Coord { x: 101, y: 103 };
    robots.iter_mut().for_each(|x| x.advance(frame, 100));
    let mid_x = frame.x / 2;
    let mid_y = frame.y / 2;
    let mut count = HashMap::<_, i64>::new();
    for Robot { pos, .. } in robots {
        let x = match pos.x.cmp(&mid_x) {
            Ordering::Less => false,
            Ordering::Equal => continue,
            Ordering::Greater => true,
        };
        let y = match pos.y.cmp(&mid_y) {
            Ordering::Less => false,
            Ordering::Equal => continue,
            Ordering::Greater => true,
        };
        *count.entry((x, y)).or_default() += 1;
    }
    let total: i64 = count.values().product();
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "222208000\n");
