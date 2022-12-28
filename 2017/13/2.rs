use std::io::BufRead;

#[derive(Debug)]
struct Layer {
    depth: u32,
    modulo: u32,
}

fn main() {
    let stdin = std::io::stdin();
    let mut layers = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split(": ").collect::<Vec<_>>();
        assert_eq!(line.len(), 2);
        let depth: u32 = line[0].parse().unwrap();
        let range: u32 = line[1].parse().unwrap();
        let modulo = 2 * (range - 1);
        layers.push(Layer { depth, modulo });
    }
    for delay in 0 .. {
        let pass = |&Layer { depth, modulo }| (delay + depth) % modulo != 0;
        if layers.iter().all(pass) {
            println!("{}", delay);
            return;
        }
    }
}
