use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
    (0 .. rock.len()).any(|i| tower.get(pos + i).is_some_and(|x| x & rock[i] >> shift != 0))
}

fn merge(tower: &mut Vec<u8>, pos: usize, rock: &[u8], shift: usize) {
    for i in 0 .. rock.len() {
        if pos + i == tower.len() {
            tower.push(0);
        }
        tower[pos + i] |= rock[i] >> shift;
    }
}

fn fall(
    next_gas: &mut usize, next_rock: &mut usize, gas: &[u8], tower: &mut Vec<u8>,
) -> (usize, usize) {
    let mut pos = tower.len() + 3;
    let (rock, max_shift) = ROCKS[*next_rock];
    *next_rock = (*next_rock + 1) % ROCKS.len();
    let mut shift = 2;
    while !collide(tower, pos, rock, shift) {
        let attempt = match gas[*next_gas] {
            b'<' if shift > 0 => Some(shift - 1),
            b'>' if shift < max_shift => Some(shift + 1),
            _ => None,
        };
        *next_gas = (*next_gas + 1) % gas.len();
        if let Some(attempt) = attempt {
            if !collide(tower, pos, rock, attempt) {
                shift = attempt;
            }
        }
        pos -= 1;
    }
    let len = tower.len();
    merge(tower, pos + 1, rock, shift);
    (len - pos - 1, shift)
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut gas = Vec::new();
    input.read_to_end(&mut gas)?;
    assert_eq!(gas.pop(), Some(0x0a));
    let mut tower = vec![0x7f];
    let mut next_gas = 0;
    let mut next_rock = 0;
    let mut seen = HashMap::new();
    let mut last_update = 0;
    let mut time = 0;
    while time - last_update <= seen.len() {
        let key = (next_gas, next_rock);
        let value = fall(&mut next_gas, &mut next_rock, &gas, &mut tower);
        match seen.entry(key) {
            Entry::Occupied(x) if *x.get() == value => (),
            x => {
                last_update = time;
                x.insert_entry(value);
            }
        }
        time += 1;
    }
    let save_time = time;
    let save_len = tower.len();
    let save_gas = next_gas;
    let save_rock = next_rock;
    loop {
        let key = (next_gas, next_rock);
        assert_eq!(fall(&mut next_gas, &mut next_rock, &gas, &mut tower), seen[&key]);
        time += 1;
        if next_gas == save_gas && next_rock == save_rock {
            break;
        }
    }
    let diff_time = time - save_time;
    let diff_len = tower.len() - save_len;
    const END_TIME: usize = 1_000_000_000_000;
    let count = (END_TIME - time) / diff_time;
    time += count * diff_time;
    while time < END_TIME {
        let key = (next_gas, next_rock);
        assert_eq!(fall(&mut next_gas, &mut next_rock, &gas, &mut tower), seen[&key]);
        time += 1;
    }
    writeln!(output, "{}", tower.len() - 1 + count * diff_len)?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "1560919540245\n");
