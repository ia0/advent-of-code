use std::io::BufRead;

fn main() {
    let mut frequency = 0i64;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        frequency += line.unwrap().parse::<i64>().unwrap();
    }
    println!("{}", frequency);
}
