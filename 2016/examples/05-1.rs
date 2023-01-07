use std::io::{Read, Write};

use anyhow::Result;
use md5::{Digest, Md5};

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut secret = String::new();
    input.read_to_string(&mut secret)?;
    let mut password = String::new();
    for index in 0 .. {
        let result = Md5::digest(format!("{secret}{index}"));
        if result[0] == 0 && result[1] == 0 && result[2] & 0xf0 == 0 {
            password.push(match result[2] & 0xf {
                x @ 0 ..= 9 => b'0' + x,
                x @ 10 ..= 15 => b'a' + x - 10,
                _ => unreachable!(),
            } as char);
            if password.len() == 8 {
                break;
            }
        }
    }
    writeln!(output, "{password}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "4543c154\n");
