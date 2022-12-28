use std::io::BufRead;

fn main() {
    let mut checksum = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let mut min = std::u64::MAX;
        let mut max = std::u64::MIN;
        for word in line.unwrap().split_whitespace() {
            let cur = word.parse().unwrap();
            if cur < min { min = cur }
            if cur > max { max = cur }
        }
        assert!(min <= max);
        checksum += max - min;
    }
    println!("{}", checksum);
}
