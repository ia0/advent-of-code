const MODULO: usize = 20201227;

fn find_secret_loop(x: usize) -> usize {
    let mut r = 0;
    let mut y = 1;
    while y != x {
        y *= 7;
        y %= MODULO;
        r += 1;
    }
    r
}

fn transform(mut x: usize, mut n: usize) -> usize {
    let mut y = 1;
    while n > 0 {
        if n % 2 == 1 {
            y *= x;
            y %= MODULO;
        }
        x *= x;
        x %= MODULO;
        n /= 2;
    }
    y
}

fn main() {
    let public_keys = [12578151, 5051300];
    let secret_loops: Vec<_> = public_keys.iter().map(|&x| find_secret_loop(x)).collect();
    println!("{}", transform(7, secret_loops.iter().product()));
}
