use std::cmp::Ordering;
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct C {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug)]
struct P {
    p: C,
    v: C,
    a: C,
}

impl C {
    fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::ops::AddAssign<C> for C {
    fn add_assign(&mut self, rhs: C) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::str::FromStr for C {
    type Err = ();

    fn from_str(input: &str) -> Result<C, ()> {
        let p: Vec<_> = input.split(',').collect();
        assert_eq!(p.len(), 3);
        let x = p[0].parse().unwrap();
        let y = p[1].parse().unwrap();
        let z = p[2].parse().unwrap();
        Ok(C { x, y, z })
    }
}

impl std::str::FromStr for P {
    type Err = ();

    fn from_str(input: &str) -> Result<P, ()> {
        assert_eq!(input.find("p=<"), Some(0));
        let x = input.find(">, v=<").unwrap();
        let y = input.find(">, a=<").unwrap();
        assert_eq!(&input[input.len() - 1 ..], ">");
        let p = input[3 .. x].parse().unwrap();
        let v = input[x + 6 .. y].parse().unwrap();
        let a = input[y + 6 .. input.len() - 1].parse().unwrap();
        Ok(P { p, v, a })
    }
}

fn aligned(p: &C, q: &C) -> bool {
    if p.x < 0 && q.x < 0 || p.x > 0 && q.x > 0 {
        return true;
    }
    if p.y < 0 && q.y < 0 || p.y > 0 && q.y > 0 {
        return true;
    }
    if p.z < 0 && q.z < 0 || p.z > 0 && q.z > 0 {
        return true;
    }
    false
}

fn main() {
    let stdin = std::io::stdin();
    let mut particules: Vec<P> = Vec::new();
    for line in stdin.lock().lines() {
        particules.push(line.unwrap().parse().unwrap());
    }
    loop {
        let mut done = true;
        for particule in particules.iter_mut() {
            particule.v += particule.a;
            particule.p += particule.v;
            if !aligned(&particule.v, &particule.a) {
                done = false;
            }
            if !aligned(&particule.p, &particule.v) {
                done = false;
            }
        }
        if done {
            break;
        }
    }
    let mut min = 0;
    for i in 1 .. particules.len() {
        match particules[i].a.norm().cmp(&particules[min].a.norm()) {
            Ordering::Less => {
                min = i;
                continue;
            }
            Ordering::Equal => (),
            Ordering::Greater => continue,
        }
        match particules[i].v.norm().cmp(&particules[min].v.norm()) {
            Ordering::Less => {
                min = i;
                continue;
            }
            Ordering::Equal => (),
            Ordering::Greater => continue,
        }
        match particules[i].p.norm().cmp(&particules[min].p.norm()) {
            Ordering::Less => {
                min = i;
                continue;
            }
            Ordering::Equal => panic!(),
            Ordering::Greater => continue,
        }
    }
    println!("{}", min);
}
