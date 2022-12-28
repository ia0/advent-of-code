use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn dist(self, other: Coord) -> u32 {
        ((other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()) as u32
    }
}

#[derive(Debug)]
struct Nanobot {
    coord: Coord,
    radius: u32,
}

fn main() {
    let stdin = std::io::stdin();
    let mut nanobots: Vec<Nanobot> = Vec::new();
    let mut largest: Option<usize> = None;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let elements: Vec<_> = line.split(|x| ['<', ',', '>', '='].contains(&x)).collect();
        let x = elements[2].parse().unwrap();
        let y = elements[3].parse().unwrap();
        let z = elements[4].parse().unwrap();
        let radius = elements[7].parse().unwrap();
        if largest.is_none() || nanobots[largest.unwrap()].radius < radius {
            largest = Some(nanobots.len());
        }
        nanobots.push(Nanobot { coord: Coord { x, y, z }, radius });
    }
    let strongest = &nanobots[largest.unwrap()];
    println!(
        "{}",
        nanobots
            .iter()
            .filter(|nanobot| nanobot.coord.dist(strongest.coord) <= strongest.radius)
            .count()
    );
}
