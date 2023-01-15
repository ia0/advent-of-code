use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};

use anyhow::Result;
use data_encoding::HEXLOWER;
use md5::{Digest, Md5};

struct Seq {
    x3: Option<u8>,
    x5: HashSet<u8>,
}

fn conv(xs: &[u8]) -> Option<u8> {
    xs[1 ..].iter().all(|&x| x == xs[0]).then_some(xs[0])
}

impl Seq {
    fn new(input: &str) -> Seq {
        let mut hash = HEXLOWER.encode(&Md5::digest(input));
        for _ in 0 .. 2016 {
            hash = HEXLOWER.encode(&Md5::digest(&hash));
        }
        let hash = hash.into_bytes();
        Seq { x3: hash.windows(3).find_map(conv), x5: hash.windows(5).filter_map(conv).collect() }
    }
}

fn update(pool: &mut HashMap<usize, Seq>, salt: &str, index: usize) {
    let Seq { x3, x5 } = Seq::new(&format!("{salt}{index}"));
    assert!(pool.insert(index, Seq { x3, x5: HashSet::new() }).is_none());
    for i in index.saturating_sub(1000) .. index {
        pool.get_mut(&i).unwrap().x5.extend(x5.iter());
    }
}

fn solve(mut input: impl Read, mut output: impl Write) -> Result<()> {
    let mut salt = String::new();
    input.read_to_string(&mut salt)?;
    let mut pool = HashMap::new();
    for index in 0 ..= 1000 {
        update(&mut pool, &salt, index);
    }
    let mut index = 0;
    for _ in 0 .. 64 {
        loop {
            let Seq { x3, x5 } = pool.remove(&index).unwrap();
            index += 1;
            update(&mut pool, &salt, index + 1000);
            if x3.map_or(false, |x3| x5.contains(&x3)) {
                break;
            }
        }
    }
    writeln!(output, "{}", index - 1)?;
    Ok(())
}

adventofcode::main!(solve("examples/14.txt") == "22045\n");
