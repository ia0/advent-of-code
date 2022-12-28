use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn orig() -> Coord {
        Coord { x: 0, y: 0, z: 0 }
    }

    fn dist(self, other: Coord) -> u32 {
        ((other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs()) as u32
    }

    fn cost(self) -> u32 {
        self.dist(Coord::orig())
    }
}

#[derive(Clone, Copy, Debug)]
struct Nanobot {
    coord: Coord,
    radius: u32,
}

impl Nanobot {
    fn in_radius(self, coord: Coord) -> bool {
        self.coord.dist(coord) <= self.radius
    }
}

struct Nanobots(Vec<Nanobot>);

impl Nanobots {
    fn score(&self, coord: Coord) -> usize {
        self.0.iter().filter(|n| n.in_radius(coord)).count()
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Fitness {
    negscore: i32,
    cost: i32,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Individual {
    fitness: Fitness,
    coord: Coord,
}

impl Individual {
    fn new(nanobots: &Nanobots, coord: Coord) -> Individual {
        let negscore = -(nanobots.score(coord) as i32);
        let cost = coord.cost() as i32;
        let fitness = Fitness { negscore, cost };
        Individual { fitness, coord }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut nanobots = Nanobots(Vec::new());
    let mut largest: Option<usize> = None;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let elements: Vec<_> = line.split(|x| ['<', ',', '>', '='].contains(&x)).collect();
        let x = elements[2].parse().unwrap();
        let y = elements[3].parse().unwrap();
        let z = elements[4].parse().unwrap();
        let radius = elements[7].parse().unwrap();
        if largest.is_none() || nanobots.0[largest.unwrap()].radius < radius {
            largest = Some(nanobots.0.len());
        }
        nanobots.0.push(Nanobot { coord: Coord { x, y, z }, radius });
    }
    println!("Found {} nanobots.", nanobots.0.len());
    let mut population: Vec<_> =
        nanobots.0.iter().map(|&n| Individual::new(&nanobots, n.coord)).collect();
    loop {
        population.sort();
        println!(
            "{} {} {}",
            population.len(),
            -population[0].fitness.negscore,
            population[0].fitness.cost
        );
        population.truncate(100);
        for i in 0 .. 20 {
            for j in 0 .. 20 {
                if i == j {
                    continue;
                }
                let Individual { coord: a, .. } = population[i];
                let Individual { coord: b, .. } = population[j];
                population.push(Individual::new(&nanobots, Coord { x: b.x, ..a }));
                population.push(Individual::new(&nanobots, Coord { y: b.y, ..a }));
                population.push(Individual::new(&nanobots, Coord { z: b.z, ..a }));
                population.push(Individual::new(
                    &nanobots,
                    Coord { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2, z: (a.z + b.z) / 2 },
                ));
            }
        }
        for i in 0 .. 20 {
            let Individual { coord: a, .. } = population[i];
            for m in [1, 10, 100, 1000, 10000, 100000].iter() {
                population.push(Individual::new(&nanobots, Coord { x: a.x - m, ..a }));
                population.push(Individual::new(&nanobots, Coord { x: a.x + m, ..a }));
                population.push(Individual::new(&nanobots, Coord { y: a.y - m, ..a }));
                population.push(Individual::new(&nanobots, Coord { y: a.y + m, ..a }));
                population.push(Individual::new(&nanobots, Coord { z: a.z - m, ..a }));
                population.push(Individual::new(&nanobots, Coord { z: a.z + m, ..a }));
            }
        }
    }
}
