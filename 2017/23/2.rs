#![feature(core_intrinsics)]
#![feature(inclusive_range_syntax)]
#![feature(iterator_step_by)]

fn main() {
    let start = 109900;
    let end: u32 = 126900;
    let limit = 1 << (33 - unsafe { std::intrinsics::ctlz(end) }) / 2;
    let mut primes = Vec::new();
    'prime: for i in 2 ..= limit {
        for prime in primes.iter() {
            if i % prime == 0 {
                continue 'prime;
            }
        }
        primes.push(i);
    }
    let mut count = 0;
    'main: for i in (start ..= end).step_by(17) {
        for prime in primes.iter() {
            if i % prime == 0 {
                count += 1;
                continue 'main;
            }
        }
    }
    println!("{}", count);
}
