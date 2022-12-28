#![feature(range_contains)]

use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut instructions: Vec<i64> = Vec::new();
    for line in stdin.lock().lines() {
        instructions.push(line.unwrap().parse().unwrap());
    }
    let mut index: i64 = 0;
    let mut count: u64 = 0;
    while (0 .. instructions.len() as i64).contains(index) {
        let offset = &mut instructions[index as usize];
        index += *offset;
        if *offset >= 3 {
            *offset -= 1;
        } else {
            *offset += 1;
        }
        count += 1;
    }
    println!("{}", count);
}
