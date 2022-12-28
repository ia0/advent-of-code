use std::io::BufRead;

fn compute_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}

fn main() {
    let mut mass = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        mass.push(line.unwrap().parse::<u64>().unwrap());
    }
    println!("{}", mass.into_iter().map(compute_fuel).sum::<u64>());
}
