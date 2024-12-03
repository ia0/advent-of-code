use std::io::{Read, Write};

use anyhow::Result;

enum State {
    CNmd,
    CmNu,
    CmuNl,
    CmulNy,
    CmulyNxc { c: u8, n: u32 },
    CmulyxcNxz { c: u8, n: u32, m: u32 },
    CdNo,
    CdoNny,
    CdoyNz,
    CdonNa,
    CdonaNt,
    CdonatNy,
    CdonatyNz,
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    let mut enabled = true;
    let mut state = State::CNmd;
    loop {
        let mut byte = 0;
        if input.read(std::slice::from_mut(&mut byte))? == 0 {
            break;
        }
        state = match (state, byte) {
            (State::CNmd, b'm') => State::CmNu,
            (State::CmNu, b'u') => State::CmuNl,
            (State::CmuNl, b'l') => State::CmulNy,
            (State::CmulNy, b'(') => State::CmulyNxc { c: 0, n: 0 },
            (State::CmulyNxc { c, n }, x @ b'0' ..= b'9') if c < 3 => {
                State::CmulyNxc { c: c + 1, n: n * 10 + (x - b'0') as u32 }
            }
            (State::CmulyNxc { c, n }, b',') if 0 < c => State::CmulyxcNxz { c: 0, n, m: 0 },
            (State::CmulyxcNxz { c, n, m }, x @ b'0' ..= b'9') if c < 3 => {
                State::CmulyxcNxz { c: c + 1, n, m: m * 10 + (x - b'0') as u32 }
            }
            (State::CmulyxcNxz { c, n, m }, b')') if 0 < c => {
                if enabled {
                    total += (n * m) as u64;
                }
                State::CNmd
            }
            (State::CNmd, b'd') => State::CdNo,
            (State::CdNo, b'o') => State::CdoNny,
            (State::CdoNny, b'(') => State::CdoyNz,
            (State::CdoyNz, b')') => {
                enabled = true;
                State::CNmd
            }
            (State::CdoNny, b'n') => State::CdonNa,
            (State::CdonNa, b'\'') => State::CdonaNt,
            (State::CdonaNt, b't') => State::CdonatNy,
            (State::CdonatNy, b'(') => State::CdonatyNz,
            (State::CdonatyNz, b')') => {
                enabled = false;
                State::CNmd
            }
            _ => State::CNmd,
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "98729041\n");
