fn main() {
    let mut count = 0;
    let mut a: usize = 277;
    let mut b: usize = 349;
    for _ in 0 .. 5 * 1000 * 1000 {
        loop {
            a = (16807 * a) % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = (48271 * b) % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }
        count += (a & 0xffff == b & 0xffff) as usize;
    }
    println!("{}", count);
}
