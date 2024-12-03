use std::io::{Read, Write};

use anyhow::Result;

enum State {
    M,
    U,
    L,
    O,
    F { c: u8, n: u32 },
    S { c: u8, n: u32, m: u32 },
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    let mut state = State::M;
    loop {
        let mut byte = 0;
        if input.read(std::slice::from_mut(&mut byte))? == 0 {
            break;
        }
        state = match (state, byte) {
            (State::M, b'm') => State::U,
            (State::U, b'u') => State::L,
            (State::L, b'l') => State::O,
            (State::O, b'(') => State::F { c: 0, n: 0 },
            (State::F { c, n }, x @ b'0' ..= b'9') if c < 3 => {
                State::F { c: c + 1, n: n * 10 + (x - b'0') as u32 }
            }
            (State::F { c, n }, b',') if 0 < c => State::S { c: 0, n, m: 0 },
            (State::S { c, n, m }, x @ b'0' ..= b'9') if c < 3 => {
                State::S { c: c + 1, n, m: m * 10 + (x - b'0') as u32 }
            }
            (State::S { c, n, m }, b')') if 0 < c => {
                total += (n * m) as u64;
                State::M
            }
            _ => State::M,
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/03.txt") == "181345830\n");
