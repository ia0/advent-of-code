fn main() {
    let mut count = 0;
    let mut a: usize = 277;
    let mut b: usize = 349;
    for _ in 0 .. 40 * 1000 * 1000 {
        a = (16807 * a) % 2147483647;
        b = (48271 * b) % 2147483647;
        count += (a & 0xffff == b & 0xffff) as usize;
    }
    println!("{}", count);
}
