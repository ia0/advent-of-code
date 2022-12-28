use std::collections::HashMap;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
enum S {
    C,
    W,
    I,
    F,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct P {
    x: i64,
    y: i64,
}

fn main() {
    let stdin = std::io::stdin();
    let mut map = HashMap::new();
    let mut x = 0;
    for line in stdin.lock().lines() {
        let mut y = 0;
        for c in line.unwrap().chars() {
            let p = P { x, y };
            match c {
                '.' => assert!(map.insert(p, S::C).is_none()),
                '#' => assert!(map.insert(p, S::I).is_none()),
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
    for _ in 0 .. 10000000 {
        let p = P { x, y };
        let e = map.entry(p).or_insert(S::C);
        match *e {
            S::C => {
                std::mem::swap(&mut dx, &mut dy);
                dx = -dx;
                *e = S::W;
            }
            S::W => {
                count += 1;
                *e = S::I;
            }
            S::I => {
                std::mem::swap(&mut dx, &mut dy);
                dy = -dy;
                *e = S::F;
            }
            S::F => {
                dx = -dx;
                dy = -dy;
                *e = S::C;
            }
        }
        x += dx;
        y += dy;
    }
    println!("{}", count);
}
