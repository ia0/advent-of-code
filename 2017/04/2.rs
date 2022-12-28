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
            let mut anagram = word.as_bytes().to_owned();
            anagram.sort();
            if !seen.insert(anagram) {
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
