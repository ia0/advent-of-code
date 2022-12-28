#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn dist_orig(self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }

    fn plane(self, i: usize) -> i32 {
        assert!(i < 8);
        let Coord { x, y, z } = self;
        (if i & 4 == 0 { x } else { -x })
            + (if i & 2 == 0 { y } else { -y })
            + (if i & 1 == 0 { z } else { -z })
    }
}

struct Poly {
    data: [i32; 8],
}

impl Poly {
    fn is_inside(&self, coord: Coord) -> bool {
        for i in 0 .. 8 {
            if coord.plane(i) > self.data[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let poly = Poly {
        data: [80162663, 56730696, 38893931, 15461963, -5464199, -33817817, -51809039, -80162657],
    };
    let mut best = None;
    for x in (42813428 ..= 47812313).step_by(1000) {
        for y in (20634363 ..= 23172423).step_by(1000) {
            for z in (11715980 ..= 14176812).step_by(1000) {
                let coord = Coord { x, y, z };
                if poly.is_inside(coord) {
                    let dist = coord.dist_orig();
                    if best.is_none() || dist < best.unwrap() {
                        println!("{} at {:?}", dist, coord);
                        best = Some(dist);
                    }
                    break;
                }
            }
        }
    }
}
