fn dist(x: u64, y: u64) -> u64 {
    if x > y { x - y } else { y - x }
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .map(|x| x.parse().unwrap())
        .unwrap_or(368078);
    let mut radius = 1;
    while input > radius * radius {
        radius += 2;
    }
    let mut side = radius * radius - (radius - 1);
    while input <= side {
        side -= radius - 1;
    }
    let delta = dist(input - side, radius / 2);
    println!("{}", radius / 2 + delta);
}
