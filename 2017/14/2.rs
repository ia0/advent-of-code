#![feature(inclusive_range_syntax)]

use std::io::BufRead;

fn reverse(elements: &mut [u8], position: usize, length: usize) {
    for i in 0 .. length / 2 {
        let x = (position + i) % elements.len();
        let y = (position + length - i - 1) % elements.len();
        let save = elements[x];
        elements[x] = elements[y];
        elements[y] = save;
    }
}

fn hash(key: &[u8]) -> [u8; 16] {
    let mut elements: Vec<u8> = (0 ..= 255).collect();
    let mut skip = 0;
    let mut position = 0;
    let mut lengths = key.to_owned();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    for _ in 0 .. 64 {
        for length in lengths.as_slice() {
            let length = *length as usize;
            reverse(elements.as_mut_slice(), position, length);
            position += length + skip;
            position %= elements.len();
            skip += 1;
        }
    }
    let mut hash = [0u8; 16];
    for i in 0 .. 16 {
        for j in 0 .. 16 {
            hash[i] ^= elements[16 * i + j];
        }
    }
    hash
}

#[derive(Clone, Debug, PartialEq)]
enum Group {
    Root,
    Child { parent: usize },
}

fn find(groups: &mut Vec<Group>, x: usize) -> usize {
    match groups[x] {
        Group::Root => x,
        Group::Child { parent } => {
            let root = find(groups, parent);
            groups[x] = Group::Child { parent: root };
            root
        }
    }
}

fn connect(groups: &mut Vec<Group>, x: usize, y: usize) {
    let rx = find(groups, x);
    let ry = find(groups, y);
    if rx == ry {
        return;
    }
    groups[std::cmp::max(rx, ry)] =
        Group::Child { parent: std::cmp::min(rx, ry) };
}

fn main() {
    let stdin = std::io::stdin();
    let key = stdin.lock().lines().next().unwrap().unwrap();
    let mut used = vec![false; 128 * 128];
    let mut groups = vec![Group::Root; 128 * 128];
    for i in 0 .. 128 {
        let key = format!("{}-{}", key, i);
        let hash = hash(key.as_bytes());
        for j in 0 .. 128 {
            if hash[j / 8] & 1 << 7 - j % 8 != 0 {
                used[i * 128 + j] = true;
                if i > 0 && used[(i - 1) * 128 + j] {
                    connect(&mut groups, i * 128 + j, (i - 1) * 128 + j);
                }
                if j > 0 && used[i * 128 + j - 1] {
                    connect(&mut groups, i * 128 + j, i * 128 + j - 1);
                }
            }
        }
    }
    let mut count = 0;
    for i in 0 .. 128 * 128 {
        count += (used[i] && groups[i] == Group::Root) as usize;
    }
    println!("{}", count);
}
