use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{ensure, Context, Result};

fn vector(input: &str) -> Result<[i64; 3]> {
    let xs = input.split(", ").map(|x| Ok(x.trim().parse()?)).collect::<Result<Vec<i64>>>()?;
    ensure!(xs.iter().all(|&x| x != 0));
    xs.try_into().ok().context("bad vector")
}

#[derive(Debug)]
struct Hail {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn intersect(a: &Hail, b: &Hail) -> bool {
    let delta = (b.pos[1] - a.pos[1]) * a.vel[0] - (b.pos[0] - a.pos[0]) * a.vel[1];
    let deltb = (a.pos[1] - b.pos[1]) * b.vel[0] - (a.pos[0] - b.pos[0]) * b.vel[1];
    let cross = a.vel[1] * b.vel[0] - a.vel[0] * b.vel[1];
    if cross == 0 {
        assert!(delta != 0);
        return false;
    }
    if delta / cross.signum() < 0 || deltb / -cross.signum() < 0 {
        return false;
    }
    let x = b.vel[0] as f64 * delta as f64 / (cross as f64) + b.pos[0] as f64;
    let y = b.vel[1] as f64 * delta as f64 / (cross as f64) + b.pos[1] as f64;
    (MIN ..= MAX).contains(&x) && (MIN ..= MAX).contains(&y)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut hails = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (pos, vel) = line.split_once(" @ ").context("no @")?;
        hails.push(Hail { pos: vector(pos)?, vel: vector(vel)? });
    }
    let mut total = 0;
    for i in 0 .. hails.len() {
        for j in i + 1 .. hails.len() {
            total += intersect(&hails[i], &hails[j]) as usize;
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

const MIN: f64 = 200000000000000.;
const MAX: f64 = 400000000000000.;
adventofcode::main!(solve("examples/24.txt") == "15262\n");
