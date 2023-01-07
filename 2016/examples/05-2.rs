use std::io::{Read, Write};

use anyhow::Result;
use md5::{Digest, Md5};

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut secret = String::new();
    input.read_to_string(&mut secret)?;
    let mut pwd = vec![None; 8];
    for index in 0 .. {
        let xs = Md5::digest(format!("{secret}{index}"));
        if xs[0] == 0 && xs[1] == 0 && xs[2] < 8 && pwd[xs[2] as usize].is_none() {
            pwd[xs[2] as usize] = Some(match xs[3] >> 4 {
                x @ 0 ..= 9 => b'0' + x,
                x @ 10 ..= 15 => b'a' + x - 10,
                _ => unreachable!(),
            } as char);
            if pwd.iter().all(|x| x.is_some()) {
                break;
            }
        }
    }
    let password = pwd.into_iter().map(|x| x.unwrap()).collect::<String>();
    writeln!(output, "{password}")?;
    Ok(())
}

adventofcode::main!(solve("examples/05.txt") == "1050cbbd\n");
