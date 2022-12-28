use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let input = File::open("examples/02.txt").unwrap();
    let regex = Regex::new("([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)").unwrap();
    let mut count = 0;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let captures = regex.captures(&line).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), &line);
        let min: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let letter = captures.get(3).unwrap().as_str().as_bytes();
        assert_eq!(letter.len(), 1);
        let letter = letter[0];
        let password = captures.get(4).unwrap().as_str().as_bytes();
        count += ((password[min - 1] == letter) != (password[max - 1] == letter)) as usize;
    }
    println!("{}", count);
}
