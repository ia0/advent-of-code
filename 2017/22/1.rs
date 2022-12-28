use std::collections::HashSet;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct C {
    x: i64,
    y: i64,
}

fn main() {
    let stdin = std::io::stdin();
    let mut map = HashSet::new();
    let mut x = 0;
    for line in stdin.lock().lines() {
        let mut y = 0;
        for c in line.unwrap().chars() {
            match c {
                '.' => (),
                '#' => assert!(map.insert(C { x, y })),
                _ => panic!(),
            }
            y += 1;
        }
        x += 1;
    }
    x /= 2;
    let mut y = x;
    let mut dx = -1;
    let mut dy = 0;
    let mut count = 0;
    for _ in 0 .. 10000 {
        let c = C { x, y };
        std::mem::swap(&mut dx, &mut dy);
        if map.contains(&c) {
            dy = -dy;
            assert!(map.remove(&c));
        } else {
            dx = -dx;
            count += 1;
            assert!(map.insert(c));
        }
        x += dx;
        y += dy;
    }
    println!("{}", count);
}
