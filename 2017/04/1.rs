use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut count = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut seen = HashSet::new();
        let mut valid = true;
        for word in line.split_whitespace() {
            if !seen.insert(word) {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1;
        }
    }
    println!("{}", count);
}
