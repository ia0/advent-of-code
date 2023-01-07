use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use adventofcode::Coord;
use anyhow::{bail, Result};

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut keypad = HashMap::new();
    keypad.insert(Coord { x: 0, y: -2 }, '1');
    keypad.insert(Coord { x: -1, y: -1 }, '2');
    keypad.insert(Coord { x: 0, y: -1 }, '3');
    keypad.insert(Coord { x: 1, y: -1 }, '4');
    keypad.insert(Coord { x: -2, y: 0 }, '5');
    keypad.insert(Coord { x: -1, y: 0 }, '6');
    keypad.insert(Coord { x: 0, y: 0 }, '7');
    keypad.insert(Coord { x: 1, y: 0 }, '8');
    keypad.insert(Coord { x: 2, y: 0 }, '9');
    keypad.insert(Coord { x: -1, y: 1 }, 'A');
    keypad.insert(Coord { x: 0, y: 1 }, 'B');
    keypad.insert(Coord { x: 1, y: 1 }, 'C');
    keypad.insert(Coord { x: 0, y: 2 }, 'D');
    let mut code = String::new();
    let mut pos = Coord::default();
    for line in BufReader::new(input).lines() {
        for c in line?.chars() {
            let dir = match c {
                'U' => Coord { x: 0, y: -1 },
                'D' => Coord { x: 0, y: 1 },
                'L' => Coord { x: -1, y: 0 },
                'R' => Coord { x: 1, y: 0 },
                _ => bail!("bad instruction {c}"),
            };
            let next = pos + dir;
            if keypad.contains_key(&next) {
                pos = next;
            }
        }
        code.push(keypad[&pos]);
    }
    writeln!(output, "{code}")?;
    Ok(())
}

adventofcode::main!(solve("examples/02.txt") == "446A6\n");
