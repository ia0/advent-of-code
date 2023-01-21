use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use regex::Regex;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut pass = b"abcdefgh".to_vec();
    let swap_pos = Regex::new("^swap position (.*) with position (.*)$")?;
    let swap_let = Regex::new("^swap letter (.*) with letter (.*)$")?;
    let rot_dir = Regex::new("^rotate (.*) (.*) steps?$")?;
    let rot_let = Regex::new("^rotate based on position of letter (.*)$")?;
    let reverse = Regex::new("^reverse positions (.*) through (.*)$")?;
    let move_ = Regex::new("^move position (.*) to position (.*)$")?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        if let Some(captures) = swap_pos.captures(&line) {
            pass.swap(captures[1].parse()?, captures[2].parse()?);
        } else if let Some(captures) = swap_let.captures(&line) {
            let x = pass.iter().position(|&x| x == captures[1].as_bytes()[0]).unwrap();
            let y = pass.iter().position(|&y| y == captures[2].as_bytes()[0]).unwrap();
            pass.swap(x, y);
        } else if let Some(captures) = rot_dir.captures(&line) {
            let steps: usize = captures[2].parse()?;
            match &captures[1] {
                "left" => pass.rotate_left(steps),
                "right" => pass.rotate_right(steps),
                _ => unreachable!(),
            }
        } else if let Some(captures) = rot_let.captures(&line) {
            let pos = pass.iter().position(|&x| x == captures[1].as_bytes()[0]).unwrap();
            let steps = (1 + pos + (4 <= pos) as usize) % pass.len();
            pass.rotate_right(steps);
        } else if let Some(captures) = reverse.captures(&line) {
            pass[captures[1].parse()? ..= captures[2].parse()?].reverse();
        } else if let Some(captures) = move_.captures(&line) {
            let letter = pass.remove(captures[1].parse()?);
            pass.insert(captures[2].parse()?, letter);
        } else {
            unreachable!();
        }
    }
    writeln!(output, "{}", std::str::from_utf8(&pass)?)?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "ghfacdbe\n");
