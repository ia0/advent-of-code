use std::fs::File;
use std::io::{BufRead, BufReader};

struct Slope {
    dx: usize,
    dy: usize,
    count: usize,
}

impl Slope {
    fn new(dx: usize, dy: usize) -> Slope {
        Slope { dx, dy, count: 0 }
    }
}

fn main() {
    let input = File::open("examples/03.txt").unwrap();
    let mut slopes =
        [Slope::new(1, 1), Slope::new(3, 1), Slope::new(5, 1), Slope::new(7, 1), Slope::new(1, 2)];
    for (y, line) in BufReader::new(input).lines().enumerate() {
        let line = line.unwrap().into_bytes();
        for slope in &mut slopes {
            if y % slope.dy != 0 {
                continue;
            }
            let x = (y / slope.dy * slope.dx) % line.len();
            slope.count += (line[x] == b'#') as usize;
        }
    }
    println!("{}", slopes.iter().map(|slope| slope.count).product::<usize>());
}
