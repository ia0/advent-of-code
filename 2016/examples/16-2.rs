use std::io::{Read, Write};

use anyhow::Result;

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut state = Vec::new();
    input.read_to_end(&mut state)?;
    const LIMIT: usize = 35651584;
    while state.len() < LIMIT {
        let len = state.len();
        state.push(b'0');
        for i in (0 .. len).rev() {
            state.push(state[i] ^ 1);
        }
    }
    state.truncate(LIMIT);
    while state.len() & 1 == 0 {
        state = state.chunks_exact(2).map(|x| b'0' + (x[0] == x[1]) as u8).collect();
    }
    writeln!(output, "{}", std::str::from_utf8(&state)?)?;
    Ok(())
}

adventofcode::main!(solve("examples/16.txt") == "01101011101100011\n");
