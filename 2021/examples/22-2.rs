use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug, Clone)]
enum Space {
    Unknown,
    End(bool),
    Nest(Vec<(i64, Space)>),
}

fn split(map: &mut Vec<(i64, Space)>, val: i64) -> usize {
    let i = map.partition_point(|&(x, _)| x < val);
    if i == map.len() {
        map.push((val, Space::Unknown));
    } else if map[i].0 > val {
        let space = if i > 0 { map[i - 1].1.clone() } else { Space::Unknown };
        map.insert(i, (val, space));
    } else {
        assert_eq!(map[i].0, val);
    }
    i
}

impl Space {
    fn split(&mut self, min: i64, max: i64) -> Vec<&mut Space> {
        if matches!(self, Space::Unknown) {
            *self = Space::Nest(Vec::new());
        }
        let map = match self {
            Space::Nest(x) => x,
            _ => unreachable!(),
        };
        let imin = split(map, min);
        let ilim = split(map, max + 1);
        assert!(imin < ilim);
        map[imin .. ilim].iter_mut().map(|(_, x)| x).collect()
    }

    fn write(&mut self, on: bool) {
        assert!(matches!(self, Space::Unknown | Space::End(_)));
        *self = Space::End(on);
    }

    fn count(&self, factor: i64) -> i64 {
        let map = match self {
            Space::Nest(x) => x,
            Space::End(true) => return factor,
            _ => return 0,
        };
        assert!(matches!(map.last().unwrap().1, Space::Unknown));
        map.windows(2).map(|x| x[0].1.count(factor * (x[1].0 - x[0].0))).sum()
    }
}

fn main() {
    let input = File::open("examples/22.txt").unwrap();
    let regex = Regex::new(r#"(-?[0-9]+)..(-?[0-9]+)"#).unwrap();
    let mut space = Space::Unknown;
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut spaces = vec![&mut space];
        for c in regex.captures_iter(&line) {
            let mut new = Vec::new();
            for space in spaces {
                new.extend(space.split(c[1].parse().unwrap(), c[2].parse().unwrap()));
            }
            spaces = new;
        }
        for space in spaces {
            space.write(line.as_bytes()[1] == b'n');
        }
    }
    println!("{}", space.count(1));
}
