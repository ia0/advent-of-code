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

fn main() {
    let stdin = std::io::stdin();
    let key = stdin.lock().lines().next().unwrap().unwrap();
    let mut count = 0;
    for i in 0 .. 128 {
        let key = format!("{}-{}", key, i);
        for byte in hash(key.as_bytes()).iter() {
            for i in 0 .. 8 {
                count += (byte & 1 << i != 0) as usize;
            }
        }
    }
    println!("{}", count);
}
