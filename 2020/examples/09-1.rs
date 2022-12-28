use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("examples/09.txt").unwrap();
    let preamble = 25;
    let input: Vec<i64> =
        BufReader::new(input).lines().map(|x| x.unwrap().parse().unwrap()).collect();
    'main: for i in preamble .. input.len() {
        let prev = &input[i - preamble ..][.. preamble];
        for j in 0 .. prev.len() - 1 {
            for k in j + 1 .. prev.len() {
                if prev[j] + prev[k] == input[i] {
                    continue 'main;
                }
            }
        }
        println!("{}", input[i]);
        return;
    }
}
