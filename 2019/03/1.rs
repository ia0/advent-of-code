use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut circuit: HashMap<(i32, i32), i32> = HashMap::new();
    let mut wire = 1;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut x = 0;
        let mut y = 0;
        for word in line.split(',') {
            let (dir, cnt) = word.split_at(1);
            let (dx, dy) = match dir.as_bytes()[0] {
                b'U' => (0, 1),
                b'D' => (0, -1),
                b'L' => (-1, 0),
                b'R' => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0 .. cnt.parse().unwrap() {
                x += dx;
                y += dy;
                *circuit.entry((x, y)).or_default() |= wire;
            }
        }
        wire <<= 1;
    }
    assert_eq!(wire, 4);
    println!(
        "{}",
        circuit
            .iter()
            .filter(|&(_, &v)| v == 3)
            .map(|((x, y), _)| x.abs() + y.abs())
            .filter(|&d| d != 0)
            .min()
            .unwrap()
    );
}
