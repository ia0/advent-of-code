use std::collections::{HashMap, HashSet};
use std::io::BufRead;

struct Count {
    num_objects: usize,
    num_orbits: usize,
}

fn id(x: &[u8]) -> usize {
    assert_eq!(x.len(), 3);
    let mut r = 0;
    for i in 0 .. x.len() {
        r *= 256;
        r += x[i] as usize;
    }
    r
}

fn count(
    orbits: &HashMap<usize, usize>, rev_orbits: &HashMap<usize, HashSet<usize>>, root: usize,
) -> Count {
    let mut num_objects = 1;
    let mut num_orbits = 0;
    if let Some(children) = rev_orbits.get(&root) {
        for &child in children {
            let count = count(orbits, rev_orbits, child);
            num_objects += count.num_objects;
            num_orbits += count.num_orbits + count.num_objects;
        }
    }
    Count { num_objects, num_orbits }
}

fn main() {
    let mut orbits = HashMap::new();
    let mut rev_orbits: HashMap<usize, HashSet<usize>> = HashMap::new();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let a = id(&line.as_bytes()[.. 3]);
        let b = id(&line.as_bytes()[4 ..]);
        assert!(orbits.insert(b, a).is_none());
        assert!(rev_orbits.entry(a).or_default().insert(b));
    }
    println!("{}", count(&orbits, &rev_orbits, id(b"COM")).num_orbits);
}
