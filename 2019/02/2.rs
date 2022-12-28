use std::io::BufRead;

fn execute(p: &[u64], x: u64, y: u64) -> u64 {
    let mut p = p.to_vec();
    p[1] = x;
    p[2] = y;
    let mut i = 0;
    while p[i] != 99 {
        let op = match p[i] {
            1 => <u64 as std::ops::Add<u64>>::add,
            2 => <u64 as std::ops::Mul<u64>>::mul,
            _ => unreachable!(),
        };
        let d = p[i + 3] as usize;
        p[d] = op(p[p[i + 1] as usize], p[p[i + 2] as usize]);
        i += 4;
    }
    p[0]
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let program: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    for x in 0 .. 100 {
        for y in 0 .. 100 {
            if execute(&program, x, y) == 19690720 {
                println!("{}", 100 * x + y);
                return;
            }
        }
    }
    unreachable!();
}
