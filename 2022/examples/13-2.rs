use std::cmp::Ordering;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;

use anyhow::{ensure, Error, Result};

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut input = input.as_bytes();
        let result = Packet::parse_packet(&mut input)?;
        ensure!(input.is_empty(), "trailing input");
        Ok(result)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::Int(x), Packet::List(y)) => Packet::cmp_list(&[Packet::Int(*x)], y),
            (Packet::List(x), Packet::Int(y)) => Packet::cmp_list(x, &[Packet::Int(*y)]),
            (Packet::List(x), Packet::List(y)) => Packet::cmp_list(x, y),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse_packet(input: &mut &[u8]) -> Result<Packet> {
        if input[0] == b'[' {
            *input = &input[1 ..];
            let mut xs = Vec::new();
            while input[0] != b']' {
                xs.push(Packet::parse_packet(input)?);
                if input[0] == b',' {
                    *input = &input[1 ..];
                }
            }
            *input = &input[1 ..];
            Ok(Packet::List(xs))
        } else {
            let mut x = 0;
            while !input.is_empty() && input[0].is_ascii_digit() {
                x = 10 * x + (input[0] - b'0') as usize;
                *input = &input[1 ..];
            }
            Ok(Packet::Int(x))
        }
    }

    fn cmp_list(xs: &[Packet], ys: &[Packet]) -> Ordering {
        for i in 0 .. xs.len() {
            if i == ys.len() {
                return Ordering::Greater;
            }
            match xs[i].cmp(&ys[i]) {
                Ordering::Equal => continue,
                x => return x,
            }
        }
        xs.len().cmp(&ys.len())
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let delim2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let delim6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    let mut packets = vec![delim2.clone(), delim6.clone()];
    for line in BufReader::new(input).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        packets.push(line.parse()?);
    }
    packets.sort();
    let pos2 = packets.iter().position(|x| x == &delim2).unwrap() + 1;
    let pos6 = packets.iter().position(|x| x == &delim6).unwrap() + 1;
    writeln!(output, "{}", pos2 * pos6)?;
    Ok(())
}

adventofcode::main!(solve("examples/13.txt") == "22600\n");
