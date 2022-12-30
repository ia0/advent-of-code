use std::io::{Read, Write};

use anyhow::Result;
use md5::{Digest, Md5};

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut secret = String::new();
    input.read_to_string(&mut secret)?;
    for answer in 1 .. {
        let mut result = Md5::digest(format!("{secret}{answer}"));
        result[2] &= 0xf0;
        if result[.. 3] == [0; 3] {
            writeln!(output, "{answer}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "254575\n");
