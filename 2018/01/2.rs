use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut frequency = 0i64;
    let mut seen = HashSet::new();
    let mut delta = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        delta.push(line.unwrap().parse::<i64>().unwrap());
    }
    for delta in delta.iter().cycle() {
        if !seen.insert(frequency) {
            println!("{}", frequency);
            return;
        }
        frequency += delta;
    }
}
