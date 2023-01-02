#![feature(slice_group_by)]

use std::io::{Read, Write};

use anyhow::Result;

fn valid(pwd: [u8; 8]) -> bool {
    let mut pair = None;
    for x in pwd.windows(2) {
        if x[0] != x[1] {
            continue;
        }
        if pair.map_or(false, |y| x[0] != y) {
            return pwd.windows(3).any(|x| x[2] == x[1] + 1 && x[1] == x[0] + 1);
        }
        pair = Some(x[0]);
    }
    false
}

fn next(pwd: &mut [u8; 8]) {
    for x in pwd.iter_mut().rev() {
        match *x {
            b'a' ..= b'g' | b'j' | b'm' | b'p' ..= b'y' => *x += 1,
            b'h' | b'k' | b'n' => *x += 2,
            b'z' => {
                *x = b'a';
                continue;
            }
            _ => unreachable!(),
        }
        break;
    }
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut pwd = Vec::new();
    input.read_to_end(&mut pwd)?;
    assert_eq!(pwd.len(), 8);
    let mut pwd = <[u8; 8]>::try_from(pwd).unwrap();
    loop {
        next(&mut pwd);
        if valid(pwd) {
            break;
        }
    }
    writeln!(output, "{}", std::str::from_utf8(&pwd)?)?;
    Ok(())
}

adventofcode::main!(solve("examples/11.txt") == "hepxxyzz\n");
