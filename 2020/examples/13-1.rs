use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/13.txt").unwrap();
    let lines: Vec<_> = BufReader::new(input).lines().map(|x| x.unwrap()).collect();
    assert_eq!(lines.len(), 2);
    let start_time: i64 = lines[0].parse().unwrap();
    println!(
        "{}",
        lines[1]
            .split(',')
            .filter(|&x| x != "x")
            .map(|x| x.parse().unwrap())
            .map(|x| ((-start_time).rem_euclid(x), x))
            .min()
            .map(|(x, y)| x * y)
            .unwrap()
    );
}
