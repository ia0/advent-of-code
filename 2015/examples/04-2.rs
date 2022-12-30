use std::io::{Read, Write};

use anyhow::Result;
use md5::{Digest, Md5};

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut secret = String::new();
    input.read_to_string(&mut secret)?;
    for answer in 1 .. {
        if Md5::digest(format!("{secret}{answer}"))[.. 3] == [0; 3] {
            writeln!(output, "{answer}")?;
            break;
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/04.txt") == "1038736\n");
