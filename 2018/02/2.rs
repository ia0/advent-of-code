use std::collections::HashSet;
use std::io::BufRead;

const STAR: u8 = b'*';

fn main() {
    let mut seen = HashSet::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap().into_bytes();
        for i in 0 .. line.len() {
            assert_ne!(line[i], STAR);
            let mut copy = line.clone();
            copy[i] = STAR;
            if !seen.insert(copy.clone()) {
                println!("{}", String::from_utf8(copy).unwrap());
            }
        }
    }
}
