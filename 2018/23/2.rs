use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    fn plane(self, i: usize) -> i32 {
        assert!(i < 8);
        let Coord { x, y, z } = self;
        (if i & 4 == 0 { x } else { -x })
            + (if i & 2 == 0 { y } else { -y })
            + (if i & 1 == 0 { z } else { -z })
    }

    fn orig_step(self) -> Vec<Coord> {
        let Coord { x, y, z } = self;
        let mut steps = Vec::new();
        if x > 0 {
            steps.push(Coord { x: x - 1, y, z });
        }
        if x < 0 {
            steps.push(Coord { x: x + 1, y, z });
        }
        if y > 0 {
            steps.push(Coord { x, y: y - 1, z });
        }
        if y < 0 {
            steps.push(Coord { x, y: y + 1, z });
        }
        if z > 0 {
            steps.push(Coord { x, y, z: z - 1 });
        }
        if z < 0 {
            steps.push(Coord { x, y, z: z + 1 });
        }
        steps
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

    fn vertices(self) -> Vec<Coord> {
        let Coord { x, y, z } = self.coord;
        let r = self.radius as i32;
        let mut result = Vec::new();
        result.push(Coord { x: x - r, y, z });
        result.push(Coord { x: x + r, y, z });
        result.push(Coord { x, y: y - r, z });
        result.push(Coord { x, y: y + r, z });
        result.push(Coord { x, y, z: z - r });
        result.push(Coord { x, y, z: z + r });
        result
    }
}

#[derive(Debug)]
struct Poly {
    data: [i32; 8],
}

impl Poly {
    fn new(nanobot: Nanobot) -> Poly {
        let mut data = [0; 8];
        for i in 0 .. 8 {
            data[i] = nanobot.coord.plane(i) + nanobot.radius as i32;
        }
        Poly { data }
    }

    fn add(&mut self, nanobot: Nanobot) {
        for i in 0 .. 8 {
            self.data[i] =
                std::cmp::min(self.data[i], nanobot.coord.plane(i) + nanobot.radius as i32);
        }
    }

    fn is_inside(&self, coord: Coord) -> bool {
        for i in 0 .. 8 {
            if coord.plane(i) > self.data[i] {
                return false;
            }
        }
        true
    }

    fn steps(&self, coord: Coord) -> Vec<Coord> {
        let mut result = Vec::new();
        let Coord { x, y, z } = coord;
        for i in 0 .. 8 {
            let d = (coord.plane(i) - self.data[i]).abs() / 3;
            if d == 0 {
                continue;
            }
            result.push(Coord {
                x: (if i & 4 == 0 { x + d } else { x - d }),
                y: (if i & 2 == 0 { y + d } else { y - d }),
                z: (if i & 1 == 0 { z + d } else { z - d }),
            });
        }
        result
    }

    fn dist_orig(&self, mut coord: Coord) -> u32 {
        assert!(self.is_inside(coord));
        loop {
            let mut progress = false;
            for step in self.steps(coord) {
                if !self.is_inside(step) {
                    continue;
                }
                if step.dist(Coord::orig()) < coord.dist(Coord::orig()) {
                    coord = step;
                    progress = true;
                    break;
                }
            }
            if !progress {
                break;
            }
        }
        loop {
            let mut progress = false;
            for step in coord.orig_step() {
                if self.is_inside(step) {
                    coord = step;
                    progress = true;
                    break;
                }
            }
            if !progress {
                break;
            }
        }
        coord.dist(Coord::orig())
    }
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
    let mut max_nanobots = 0;
    // let mut min_dist = None;
    for &nanobot in nanobots.iter() {
        for coord in nanobot.vertices() {
            let num_nanobots = nanobots.iter().filter(|n| n.in_radius(coord)).count();
            if num_nanobots < max_nanobots {
                continue;
            }
            if num_nanobots > max_nanobots {
                max_nanobots = num_nanobots;
                // min_dist = None;
            }
            let mut poly = Poly::new(nanobot);
            for &n in nanobots.iter() {
                if n.in_radius(coord) {
                    poly.add(n);
                }
            }
            println!("{} {:?}", max_nanobots, poly);
            println!(
                "{} <= x <= {}",
                std::cmp::max(
                    -(poly.data[4] + poly.data[7]) / 2,
                    -(poly.data[5] + poly.data[6]) / 2,
                ),
                std::cmp::min((poly.data[0] + poly.data[3]) / 2, (poly.data[1] + poly.data[2]) / 2)
            );
            println!(
                "{} <= y <= {}",
                std::cmp::max(
                    -(poly.data[2] + poly.data[7]) / 2,
                    -(poly.data[3] + poly.data[6]) / 2,
                ),
                std::cmp::min((poly.data[0] + poly.data[5]) / 2, (poly.data[1] + poly.data[4]) / 2)
            );
            println!(
                "{} <= z <= {}",
                std::cmp::max(
                    -(poly.data[1] + poly.data[7]) / 2,
                    -(poly.data[3] + poly.data[5]) / 2,
                ),
                std::cmp::min((poly.data[0] + poly.data[6]) / 2, (poly.data[2] + poly.data[4]) / 2)
            );
            // let dist = poly.dist_orig(coord);
            // if min_dist.is_none() || dist < min_dist.unwrap() {
            //     min_dist = Some(dist);
            //     println!("{} {} {:?}", max_nanobots, dist, coord);
            // }
        }
    }
    // println!("{}", min_dist.unwrap());
}
