extern crate regex;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Coord,
    vel: Coord,
}

fn gravity(this: i64, other: i64) -> i64 {
    match this.cmp(&other) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

fn main() {
    let file = File::open("examples/12.txt").unwrap();
    let coord_regex = Regex::new("<x=(.*), y=(.*), z=(.*)>").unwrap();
    let mut moons = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let c = coord_regex.captures(&line).unwrap();
        assert_eq!(c.len(), 4);
        let pos =
            Coord { x: c[1].parse().unwrap(), y: c[2].parse().unwrap(), z: c[3].parse().unwrap() };
        let vel = Coord { x: 0, y: 0, z: 0 };
        moons.push(Moon { pos, vel });
    }
    for _ in 0 .. 1000 {
        let mut new_moons = moons.clone();
        for i in 0 .. moons.len() {
            for j in 0 .. moons.len() {
                if i == j {
                    continue;
                }
                new_moons[i].vel.x += gravity(moons[i].pos.x, moons[j].pos.x);
                new_moons[i].vel.y += gravity(moons[i].pos.y, moons[j].pos.y);
                new_moons[i].vel.z += gravity(moons[i].pos.z, moons[j].pos.z);
            }
            new_moons[i].pos.x += new_moons[i].vel.x;
            new_moons[i].pos.y += new_moons[i].vel.y;
            new_moons[i].pos.z += new_moons[i].vel.z;
        }
        moons = new_moons;
    }
    let mut total = 0;
    for moon in moons {
        let potential = moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs();
        let kinetic = moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs();
        total += potential * kinetic;
    }
    println!("{}", total);
}
