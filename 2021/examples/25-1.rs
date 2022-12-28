use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn step(m: &mut Vec<Vec<u8>>, t: u8) -> bool {
    let mut todo = HashSet::new();
    let nx = m[0].len();
    let ny = m.len();
    for x in 0 .. nx {
        for y in 0 .. ny {
            let (sx, sy) = match t {
                b'>' => ((x + 1) % nx, y),
                b'v' => (x, (y + 1) % ny),
                _ => unreachable!(),
            };
            if m[y][x] == t && m[sy][sx] == b'.' {
                todo.insert((x, y, sx, sy));
            }
        }
    }
    let r = !todo.is_empty();
    for (x, y, sx, sy) in todo {
        m[y][x] = b'.';
        m[sy][sx] = t;
    }
    r
}

fn main() {
    let input = File::open("examples/25.txt").unwrap();
    let mut map: Vec<_> = BufReader::new(input).lines().map(|x| x.unwrap().into_bytes()).collect();
    assert!(map.iter().all(|x| x.len() == map[0].len()));
    let mut count = 1;
    while step(&mut map, b'>') | step(&mut map, b'v') {
        count += 1;
    }
    println!("{}", count);
}
