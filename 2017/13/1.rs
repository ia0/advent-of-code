use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut severity = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split(": ").collect::<Vec<_>>();
        assert_eq!(line.len(), 2);
        let depth: i32 = line[0].parse().unwrap();
        let range: i32 = line[1].parse().unwrap();
        assert!(range > 0);
        if depth % (2 * (range - 1)) == 0 {
            severity += depth * range;
        }
    }
    println!("{}", severity);
}
