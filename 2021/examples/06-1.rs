use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/06.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut count = vec![0usize; 9];
    for word in lines.next().unwrap().unwrap().split(',') {
        count[word.parse::<usize>().unwrap()] += 1;
    }
    assert!(lines.next().is_none());
    for _ in 0 .. 80 {
        count.rotate_left(1);
        count[6] += count[8];
    }
    println!("{}", count.iter().sum::<usize>());
}
