use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

type Name = internment::Intern<String>;

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut repl = HashMap::<Name, HashSet<Name>>::new();
    let mut lines = BufReader::new(input).lines();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (src, dst) = line.split_once(" => ").unwrap();
        assert!(repl.entry(Name::from_ref(src)).or_default().insert(Name::from_ref(dst)));
    }
    let molecule = lines.next().unwrap()?;
    assert!(lines.next().is_none());
    let mut candidates = HashSet::new();
    for (src, dsts) in repl {
        for dst in dsts {
            let mut pieces = Vec::new();
            for piece in molecule.split(src.as_str()) {
                pieces.push(piece);
                pieces.push(src.as_str());
            }
            pieces.pop();
            for i in (1 .. pieces.len()).step_by(2) {
                pieces[i] = dst.as_str();
                candidates.insert(pieces.join(""));
                pieces[i] = src.as_str();
            }
        }
    }
    writeln!(output, "{}", candidates.len())?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "518\n");
