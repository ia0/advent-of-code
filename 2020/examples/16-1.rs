use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

use regex::Regex;

struct Field {
    #[allow(dead_code)]
    name: String,
    ranges: [Range<i32>; 2],
}

impl Field {
    fn is_valid(&self, x: i32) -> bool {
        self.ranges[0].contains(&x) || self.ranges[1].contains(&x)
    }
}

fn parse_ticket(x: &str) -> Vec<i32> {
    x.split(',').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let input = File::open("examples/16.txt").unwrap();
    let regex = Regex::new(r#"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$"#).unwrap();
    let mut fields = Vec::new();
    let mut lines = BufReader::new(input).lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let captures = regex.captures(&line).unwrap();
        fields.push(Field {
            name: captures.get(1).unwrap().as_str().to_string(),
            ranges: [
                Range {
                    start: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    end: captures.get(3).unwrap().as_str().parse::<i32>().unwrap() + 1,
                },
                Range {
                    start: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    end: captures.get(5).unwrap().as_str().parse::<i32>().unwrap() + 1,
                },
            ],
        });
    }
    assert_eq!(lines.next().unwrap().unwrap(), "your ticket:");
    let _ticket = parse_ticket(&lines.next().unwrap().unwrap());
    assert_eq!(lines.next().unwrap().unwrap(), "");
    assert_eq!(lines.next().unwrap().unwrap(), "nearby tickets:");
    let mut nearby = Vec::new();
    for line in lines {
        nearby.push(parse_ticket(&line.unwrap()));
    }
    println!(
        "{}",
        nearby
            .iter()
            .map(|x| x.iter())
            .flatten()
            .filter(|&&x| fields.iter().all(|y| !y.is_valid(x)))
            .sum::<i32>()
    );
}
