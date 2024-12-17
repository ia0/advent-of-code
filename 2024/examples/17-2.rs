use std::io::{Read, Write};

use anyhow::Result;

fn check(mut input: u64, mut expected: u64) -> bool {
    for _ in 0 .. 16 {
        if input == 0 || expected & 7 != (input ^ 3 ^ (input >> ((input & 7) ^ 5))) & 7 {
            return false;
        }
        input >>= 3;
        expected >>= 3;
    }
    true
}

fn search(input: u64, mask: u64, expected: u64, results: &mut Vec<u64>) {
    if mask == u64::MAX {
        if check(input, expected) {
            results.push(input);
        }
        return;
    }
    let n = 3 * (mask.trailing_ones() / 3);
    for i in 0 .. 8 {
        let mut input = input;
        let mut mask = mask;
        let sub_input = i << n;
        let sub_mask = 7 << n;
        if input & sub_mask & mask != sub_input & mask {
            continue;
        }
        input |= sub_input;
        mask |= sub_mask;
        let off = (i ^ 5) as u32;
        let off_input = (i ^ 3 ^ ((expected >> n) & 7)) << (n + off);
        let off_mask = 7 << (n + off);
        if input & off_mask & mask != off_input & mask {
            continue;
        }
        input |= off_input;
        mask |= off_mask;
        search(input, mask, expected, results);
    }
}

fn solve(_: impl Read, mut output: impl Write) -> Result<()> {
    let input: [u8; 16] = [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0];
    let mask = (1u64 << (3 * input.len())) - 1;
    let expected: u64 = input.iter().enumerate().map(|(i, k)| (*k as u64) << (3 * i)).sum();
    let mut results = Vec::new();
    search(0, !mask, expected, &mut results);
    results.sort();
    writeln!(output, "{}", results[0])?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "106086382266778\n");
