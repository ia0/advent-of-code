use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(x: u8) -> (bool, usize) {
    match x {
        b'(' => (true, 0),
        b'[' => (true, 1),
        b'{' => (true, 2),
        b'<' => (true, 3),
        b')' => (false, 0),
        b']' => (false, 1),
        b'}' => (false, 2),
        b'>' => (false, 3),
        _ => unreachable!(),
    }
}

fn score(xs: &str) -> usize {
    let mut stack = Vec::new();
    for &x in xs.as_bytes() {
        match parse(x) {
            (true, i) => stack.push(i),
            (false, i) => match stack.pop() {
                None => unreachable!(),
                Some(j) if i != j => return 0,
                Some(_) => (),
            },
        }
    }
    let mut score = 0;
    while let Some(i) = stack.pop() {
        score = 5 * score + i + 1;
    }
    score
}

fn main() {
    let input = File::open("examples/10.txt").unwrap();
    let input: Vec<_> = BufReader::new(input).lines().map(|x| x.unwrap()).collect();
    let mut scores: Vec<_> = input.iter().map(|xs| score(xs)).filter(|&x| x > 0).collect();
    scores.sort();
    assert_eq!(scores.len() % 2, 1);
    println!("{}", scores[scores.len() / 2]);
}
