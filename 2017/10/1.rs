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
    for length in stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
    {
        reverse(elements.as_mut_slice(), position, length);
        position += length + skip;
        position %= elements.len();
        skip += 1;
    }
    println!("{}", elements[0] * elements[1]);
}
