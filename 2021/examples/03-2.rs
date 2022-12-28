use std::collections::HashSet;
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

fn find(mut xs: HashSet<&[bool]>, b: bool) -> &[bool] {
    let mut i = 0;
    while xs.len() > 1 {
        let n = xs.len();
        let c = xs.iter().filter(|x| x[i]).count();
        let w = (2 * c >= n) == b;
        xs.retain(|x| x[i] == w);
        i += 1;
    }
    xs.iter().next().unwrap()
}

fn main() {
    let input = File::open("examples/03.txt").unwrap();
    let lines: HashSet<_> = BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().bytes().map(|x| x == b'1').collect::<Vec<_>>())
        .collect();
    let k = lines.iter().next().unwrap().len();
    assert!(lines.iter().all(|x| x.len() == k));
    let lines: HashSet<_> = lines.iter().map(|x| x as &[bool]).collect();
    let oxygen = find(lines.clone(), true);
    let co2 = find(lines, false);
    println!("{}", convert(&oxygen) * convert(&co2));
}
