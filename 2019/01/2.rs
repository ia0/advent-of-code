use std::io::BufRead;

fn compute_fuel(mass: u64) -> u64 {
    let mut total = 0;
    let mut fuel = mass;
    while fuel > 0 {
        fuel = (fuel / 3).saturating_sub(2);
        total += fuel;
    }
    total
}

fn main() {
    let mut mass = Vec::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        mass.push(line.unwrap().parse::<u64>().unwrap());
    }
    println!("{}", mass.into_iter().map(compute_fuel).sum::<u64>());
}
