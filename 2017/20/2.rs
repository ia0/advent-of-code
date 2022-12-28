use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    let mut valid = HashSet::new();
    for line in stdin.lock().lines() {
        assert!(valid.insert(particules.len()));
        particules.push(line.unwrap().parse().unwrap());
    }
    loop {
        let mut done = true;
        let mut seen = HashMap::new();
        let mut remove = HashSet::new();
        for &i in &valid {
            particules[i].v += particules[i].a;
            particules[i].p += particules[i].v;
            if let Some(k) = seen.insert(particules[i].p, i) {
                remove.insert(k);
                remove.insert(i);
            }
            if !aligned(&particules[i].v, &particules[i].a) {
                done = false;
            }
            if !aligned(&particules[i].p, &particules[i].v) {
                done = false;
            }
        }
        for i in remove {
            valid.remove(&i);
        }
        if done {
            break;
        }
    }
    println!("{}", valid.len());
}
