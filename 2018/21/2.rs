use std::collections::HashSet;

fn main() {
    let mut prev = None;
    let mut seen = HashSet::new();
    let mut r5 = 0u64;
    loop {
        let mut r1 = r5 | 0x010000;
        r5 = 0xA2F195;
        while r1 > 0 {
            r5 += r1 & 0x0000FF;
            r1 /= 0x100;
            r5 &= 0xFFFFFF;
            r5 *= 0x01016B;
            r5 &= 0xFFFFFF;
        }
        if !seen.insert(r5) {
            println!("{}", prev.unwrap());
            return;
        }
        prev = Some(r5);
    }
}
