use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_byte(input: &mut &[u8]) -> u8 {
    let result = input[0];
    *input = &input[1 ..];
    result
}

fn parse_value(input: &mut &[u8]) -> usize {
    let mut x = 0;
    while !input.is_empty() && input[0].is_ascii_digit() {
        let c = parse_byte(input);
        assert!(b'0' <= c && c <= b'9');
        x = 10 * x + (c - b'0') as usize;
    }
    x
}

fn parse_next(input: &mut &[u8]) -> usize {
    match input[0] {
        b'0' ..= b'9' => parse_value(input),
        b'(' => {
            assert_eq!(parse_byte(input), b'(');
            let r = parse_expr(input);
            assert_eq!(parse_byte(input), b')');
            r
        }
        _ => unreachable!(),
    }
}

fn parse_expr(input: &mut &[u8]) -> usize {
    let mut p = 1;
    let mut r = parse_next(input);
    while !input.is_empty() && input[0] != b')' {
        assert_eq!(parse_byte(input), b' ');
        let op = parse_byte(input);
        assert_eq!(parse_byte(input), b' ');
        let e = parse_next(input);
        match op {
            b'+' => r += e,
            b'*' => {
                p *= r;
                r = e;
            }
            _ => unreachable!(),
        }
    }
    p * r
}

fn main() {
    let input = File::open("examples/18.txt").unwrap();
    let mut sum = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut input = line.as_bytes();
        sum += parse_expr(&mut input);
        assert_eq!(input.len(), 0);
    }
    println!("{}", sum);
}
