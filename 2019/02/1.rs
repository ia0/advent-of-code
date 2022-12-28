use std::io::BufRead;

fn execute(p: &mut [u64]) {
    let mut i = 0;
    while p[i] != 99 {
        let op = match p[i] {
            1 => <u64 as std::ops::Add<u64>>::add,
            2 => <u64 as std::ops::Mul<u64>>::mul,
            _ => unreachable!(),
        };
        p[p[i + 3] as usize] = op(p[p[i + 1] as usize], p[p[i + 2] as usize]);
        i += 4;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut program: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
    program[1] = 12;
    program[2] = 2;
    execute(&mut program);
    println!("{}", program[0]);
}
