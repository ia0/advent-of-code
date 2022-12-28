use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dist(a: usize, b: usize) -> usize {
    std::cmp::max(a, b) - std::cmp::min(a, b)
}

fn fuel(count: &HashMap<usize, usize>, pos: usize) -> usize {
    let mut result = 0;
    for (&key, &count) in count {
        result += count * dist(key, pos);
    }
    result
}

fn main() {
    let input = File::open("examples/07.txt").unwrap();
    let mut lines = BufReader::new(input).lines();
    let mut count: HashMap<usize, usize> = HashMap::new();
    for word in lines.next().unwrap().unwrap().split(',') {
        *count.entry(word.parse().unwrap()).or_default() += 1;
    }
    assert!(lines.next().is_none());
    let min = *count.keys().min().unwrap();
    let max = *count.keys().max().unwrap();
    let best = (min ..= max).min_by_key(|&pos| fuel(&count, pos)).unwrap();
    println!("{}", fuel(&count, best));
}
