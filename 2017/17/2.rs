#![feature(inclusive_range_syntax)]

fn main() {
    let step = 386;
    let mut pos = 0;
    let mut next = -1;
    for value in 1 ..= 50 * 1000 * 1000 {
        pos += step;
        pos %= value;
        if pos == 0 {
            next = value;
        }
        pos += 1;
    }
    println!("{}", next);
}
