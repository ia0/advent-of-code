use std::fs::File;
use std::io::{BufRead, BufReader};

fn convert(xs: &[bool]) -> usize {
    let mut r = 0;
    for &x in xs.iter() {
        r *= 2;
        r += x as usize;
    }
    r
}

fn main() {
    let input = File::open("examples/03.txt").unwrap();
    let lines: Vec<_> =
        BufReader::new(input).lines().map(|line| line.unwrap().into_bytes()).collect();
    let n = lines.len();
    let k = lines[0].len();
    assert!(lines.iter().all(|x| x.len() == k));
    let counts: Vec<usize> =
        (0 .. k).map(|i| lines.iter().map(|x| (x[i] == b'1') as usize).sum()).collect();
    assert!(n % 2 == 1 || counts.iter().all(|&x| x != n / 2));
    let gamma: Vec<_> = counts.iter().map(|&x| x > n / 2).collect();
    let epsilon: Vec<_> = counts.iter().map(|&x| x < n / 2).collect();
    println!("{}", convert(&gamma) * convert(&epsilon));
}
