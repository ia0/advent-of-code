use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut nw: i32 = 0;
    let mut ne: i32 = 0;
    for direction in stdin.lock().lines().next().unwrap().unwrap().split(',') {
        if direction == "n" {
            nw += 1;
            ne += 1;
        } else if direction == "ne" {
            ne += 1;
        } else if direction == "se" {
            nw -= 1;
        } else if direction == "s" {
            nw -= 1;
            ne -= 1;
        } else if direction == "sw" {
            ne -= 1;
        } else {
            assert_eq!(direction, "nw");
            nw += 1;
        }
    }
    let dist = if nw * ne >= 0 {
        std::cmp::max(nw.abs(), ne.abs())
    } else {
        nw.abs() + ne.abs()
    };
    println!("{}", dist);
}
