use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use internment::Intern;
use regex::Regex;

type Name = Intern<String>;

struct Reindeer {
    speed: usize,
    fly: usize,
    rest: usize,
}

impl Reindeer {
    fn dist(&self, time: usize) -> usize {
        let total = self.fly + self.rest;
        let full = time / total;
        (full * self.fly + std::cmp::min(self.fly, time % total)) * self.speed
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let regex = Regex::new(
        r"^(.*) can fly (.*) km/s for (.*) seconds, but then must rest for (.*) seconds\.$",
    )?;
    let mut reindeers = HashMap::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let capture = regex.captures(&line).unwrap();
        assert!(reindeers
            .insert(
                Name::from_ref(&capture[1]),
                Reindeer {
                    speed: capture[2].parse()?,
                    fly: capture[3].parse()?,
                    rest: capture[4].parse()?,
                },
            )
            .is_none());
    }
    let mut scores = reindeers.keys().map(|&x| (x, 0)).collect::<HashMap<Name, usize>>();
    for dist in 1 ..= 2503 {
        let lead = reindeers.values().map(|x| x.dist(dist)).max().unwrap();
        for (name, score) in &mut scores {
            if reindeers[name].dist(dist) == lead {
                *score += 1;
            }
        }
    }
    writeln!(output, "{}", scores.values().max().unwrap())?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "1059\n");
