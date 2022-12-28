use std::io::BufRead;

fn reverse(elements: &mut [i32], position: usize, length: usize) {
    for i in 0 .. length / 2 {
        let x = (position + i) % elements.len();
        let y = (position + length - i - 1) % elements.len();
        let save = elements[x];
        elements[x] = elements[y];
        elements[y] = save;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut elements: Vec<i32> = (0 .. 256).collect();
    let mut skip = 0;
    let mut position = 0;
    let mut lengths: Vec<_> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .bytes()
        .collect();
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
    for i in 0 .. 16 {
        let mut x = 0;
        for j in 0 .. 16 {
            x ^= elements[16 * i + j];
        }
        print!("{:02x}", x);
    }
    println!("");
}
