fn main() {
    const R2: usize = 10551430;
    let mut r0 = 0;
    for r4 in 1 ..= R2 {
        if R2 % r4 == 0 {
            r0 += r4;
        }
    }
    println!("{}", r0);
}
