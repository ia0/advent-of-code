use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::{Coord, ADJACENT_PLUS};
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

    fn advance(&mut self) {
        self.pos += self.vel;
        self.pos %= FRAME;
    }
}

const FRAME: Coord = Coord { x: 101, y: 103 };

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut robots = Vec::new();
    for line in BufReader::new(input).lines() {
        robots.push(Robot::parse(&line?)?);
    }
    let mut best = 0;
    let total = 7623;
    for _i in 0 ..= total {
        let map: HashSet<Coord> = robots.iter().map(|x| x.pos).collect();
        let cur = FRAME
            .iter()
            .filter(|&pos| ADJACENT_PLUS.iter().all(|&dir| !map.contains(&(pos + dir))))
            .count();
        if best < cur {
            // adventofcode::print_set(std::io::stderr(), &map, false)?;
            // eprintln!("{_i}");
            best = cur;
        }
        robots.iter_mut().for_each(Robot::advance);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "7623\n");
