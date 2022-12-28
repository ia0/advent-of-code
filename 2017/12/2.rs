use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug)]
enum Group {
    Root { count: usize },
    Child { root: usize },
}

impl Group {
    fn count(&self) -> usize {
        match *self {
            Group::Root { count } => count,
            Group::Child { .. } => panic!(),
        }
    }
}

fn find(groups: &mut Vec<Group>, x: usize) -> usize {
    match groups[x] {
        Group::Root { .. } => x,
        Group::Child { root } => {
            let root = find(groups, root);
            groups[x] = Group::Child { root };
            root
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut groups = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split(" <-> ").collect::<Vec<_>>();
        assert_eq!(line.len(), 2);
        let i = line[0].parse().unwrap();
        let mut seen = HashSet::new();
        let mut count = 1;
        let mut root = i;
        for j in line[1].split(", ") {
            let j = j.parse().unwrap();
            if j >= i {
                continue;
            }
            let sub_root = find(&mut groups, j);
            if seen.insert(sub_root) {
                count += groups[sub_root].count();
                root = std::cmp::min(root, sub_root);
            }
        }
        if root == i {
            assert_eq!(count, 1);
            groups.push(Group::Root { count });
        } else {
            groups.push(Group::Child { root });
            for x in seen {
                if x == root {
                    groups[x] = Group::Root { count };
                } else {
                    groups[x] = Group::Child { root };
                }
            }
        }
    }
    let mut count = 0;
    for group in groups {
        match group {
            Group::Root { .. } => count += 1,
            Group::Child { .. } => (),
        }
    }
    println!("{}", count);
}
