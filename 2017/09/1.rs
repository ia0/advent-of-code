use std::io::Read;

fn main() {
    let mut depth = 1;
    let mut count = 0;
    let mut garbage = false;
    let mut escape = false;
    for byte in std::io::stdin().bytes() {
        let byte = byte.unwrap();
        if escape {
            escape = false;
            continue;
        }
        if garbage {
            match byte {
                b'!' => escape = true,
                b'>' => garbage = false,
                _ => (),
            }
        } else {
            match byte {
                b'{' => {
                    count += depth;
                    depth += 1;
                }
                b'}' => depth -= 1,
                b'<' => garbage = true,
                _ => (),
            }
        }
    }
    println!("{}", count);
}
