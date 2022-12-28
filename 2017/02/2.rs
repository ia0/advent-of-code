use std::io::BufRead;

fn main() {
    let mut checksum = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let xs: Vec<u64> = line.unwrap()
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        for i in 0 .. xs.len() {
            for j in 0 .. i {
                if xs[i] % xs[j] == 0 || xs[j] % xs[i] == 0 {
                    checksum += xs[i] / xs[j] + xs[j] / xs[i];
                }
            }
        }
    }
    println!("{}", checksum);
}
