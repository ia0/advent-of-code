use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn segment(xs: &str) -> Vec<&[u8]> {
    xs.as_bytes().chunk_by(|x, y| x.is_ascii_uppercase() && y.is_ascii_lowercase()).collect()
}

fn ok(x: &[u8]) -> bool {
    const SPECIALS: &[&[u8]] = &[b"Rn", b"Y", b"Ar"];
    !SPECIALS.contains(&x)
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (src, dst) = line.split_once(" => ").unwrap();
        let src = segment(src);
        assert_eq!(src.len(), 1);
        assert!(ok(src[0]));
        let dst = segment(dst);
        match dst.as_slice() {
            [x, y] if ok(x) && ok(y) => (),
            [x, b"Rn", y, b"Ar"] if ok(x) && ok(y) => (),
            [x, b"Rn", y, b"Y", z, b"Ar"] if ok(x) && ok(y) && ok(z) => (),
            [x, b"Rn", y, b"Y", z, b"Y", w, b"Ar"] if ok(x) && ok(y) && ok(z) && ok(w) => (),
            _ => unreachable!(),
        }
    }
    let molecule = lines.next().unwrap()?;
    assert!(lines.next().is_none());
    let molecule = segment(&molecule);
    writeln!(
        output,
        "{}",
        molecule.len()
            - molecule.iter().filter(|&x| x == b"Rn").count()
            - molecule.iter().filter(|&x| x == b"Ar").count()
            - 2 * molecule.iter().filter(|&x| x == b"Y").count()
            - 1
    )?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "200\n");
