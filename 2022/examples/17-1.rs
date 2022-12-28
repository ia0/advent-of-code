use std::io::{Read, Write};

use anyhow::Result;

const ROCKS: &[(&[u8], usize)] = &[
    (b"\x78", 3),
    (b"\x20\x70\x20", 4),
    (b"\x70\x10\x10", 4),
    (b"\x40\x40\x40\x40", 6),
    (b"\x60\x60", 5),
];

fn collide(tower: &[u8], pos: usize, rock: &[u8], shift: usize) -> bool {
    (0 .. rock.len()).any(|i| tower.get(pos + i).map_or(false, |x| x & rock[i] >> shift != 0))
}

fn merge(tower: &mut Vec<u8>, pos: usize, rock: &[u8], shift: usize) {
    for i in 0 .. rock.len() {
        if pos + i == tower.len() {
            tower.push(0);
        }
        tower[pos + i] |= rock[i] >> shift;
    }
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut gas = Vec::new();
    input.read_to_end(&mut gas)?;
    assert_eq!(gas.pop(), Some(0x0a));
    let mut tower = vec![0x7f];
    let mut next_gas = 0;
    let mut next_rock = 0;
    for _ in 0 .. 2022 {
        let mut pos = tower.len() + 3;
        let (rock, max_shift) = ROCKS[next_rock];
        next_rock = (next_rock + 1) % ROCKS.len();
        let mut shift = 2;
        while !collide(&tower, pos, rock, shift) {
            let attempt = match gas[next_gas] {
                b'<' if shift > 0 => Some(shift - 1),
                b'>' if shift < max_shift => Some(shift + 1),
                _ => None,
            };
            next_gas = (next_gas + 1) % gas.len();
            if let Some(attempt) = attempt {
                if !collide(&tower, pos, rock, attempt) {
                    shift = attempt;
                }
            }
            pos -= 1;
        }
        merge(&mut tower, pos + 1, rock, shift);
    }
    writeln!(output, "{}", tower.len() - 1)?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "3151\n");
