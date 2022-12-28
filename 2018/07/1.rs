use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut dependencies: HashMap<u8, HashSet<u8>> = HashMap::new();
    for line in stdin.lock().lines() {
        let line: Vec<_> =
            line.unwrap().bytes().filter(|x| x.is_ascii_uppercase()).skip(1).collect();
        assert_eq!(line.len(), 2);
        dependencies.entry(line[1]).or_default().insert(line[0]);
        dependencies.entry(line[0]).or_default();
    }
    let mut ready = BTreeSet::new();
    for (&step, dependencies) in dependencies.iter() {
        if dependencies.is_empty() {
            ready.insert(step);
        }
    }
    while let Some(&next) = ready.iter().next() {
        ready.remove(&next);
        print!("{}", next as char);
        for (&step, dependencies) in dependencies.iter_mut() {
            if dependencies.remove(&next) && dependencies.is_empty() {
                ready.insert(step);
            }
        }
    }
    println!();
}
