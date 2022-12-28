use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve(input: &[usize]) -> usize {
    if input.len() <= 1 {
        return 1;
    }
    let i = input.len() / 2;
    let a = solve(&input[.. i]);
    let b = solve(&input[i + 1 ..]);
    let mut c = None;
    let mut d = None;
    let mut count = a * b;
    if i > 0 && input[i - 1] + input[i] <= 3 {
        c = Some(solve(&input[.. i - 1]));
        count += c.unwrap() * b;
        if i > 1 && input[i - 2] + input[i - 1] + input[i] <= 3 {
            count += solve(&input[.. i - 2]) * b;
        }
    }
    if i < input.len() - 1 && input[i] + input[i + 1] <= 3 {
        d = Some(solve(&input[i + 2 ..]));
        count += a * d.unwrap();
        if i < input.len() - 2 && input[i] + input[i + 1] + input[i + 2] <= 3 {
            count += a * solve(&input[i + 3 ..]);
        }
    }
    if i > 0 && i < input.len() - 1 && input[i - 1] + input[i] + input[i + 1] <= 3 {
        count += c.unwrap() * d.unwrap();
    }
    count
}

fn main() {
    let input = File::open("examples/10.txt").unwrap();
    let mut input: Vec<usize> =
        BufReader::new(input).lines().map(|x| x.unwrap().parse().unwrap()).collect();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    let diffs: Vec<usize> = input.windows(2).map(|x| x[1] - x[0]).collect();
    println!("{}", solve(&diffs));
}
