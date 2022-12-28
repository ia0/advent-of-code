use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut two = 0;
    let mut three = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let mut count = HashMap::new();
        for letter in line.unwrap().chars() {
            *count.entry(letter).or_insert(0) += 1;
        }
        let mut has_two = false;
        let mut has_three = false;
        for &count in count.values() {
            has_two |= count == 2;
            has_three |= count == 3;
        }
        two += has_two as i32;
        three += has_three as i32;
    }
    println!("{}", two * three);
}
