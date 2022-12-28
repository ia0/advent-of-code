use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::mem::take;

use anyhow::Result;

#[derive(Debug, Default)]
enum After {
    #[default]
    Invalid,
    Directory(HashMap<String, After>),
    File(usize),
}

#[derive(Debug, Default)]
enum Before {
    #[default]
    Invalid,
    Root,
    Parent {
        before: Box<Before>,
        name: String,
        rest: HashMap<String, After>,
    },
}

#[derive(Debug)]
struct View {
    before: Before,
    after: After,
}

impl After {
    fn dir() -> After {
        After::Directory(HashMap::new())
    }

    fn file(size: usize) -> After {
        After::File(size)
    }
}

impl View {
    fn new() -> View {
        View { before: Before::Root, after: After::dir() }
    }

    fn cd(&mut self, name: &str) {
        if name == ".." {
            match take(&mut self.before) {
                Before::Parent { before, name, mut rest } => {
                    self.before = *before;
                    assert!(rest.insert(name, take(&mut self.after)).is_none());
                    self.after = After::Directory(rest);
                }
                _ => unreachable!(),
            }
        } else {
            match take(&mut self.after) {
                After::Directory(mut rest) => {
                    self.after = rest.remove(name).unwrap_or(After::dir());
                    self.before = Before::Parent {
                        before: Box::new(take(&mut self.before)),
                        name: name.to_string(),
                        rest,
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn dir(&mut self, name: &str) {
        match &mut self.after {
            After::Directory(content) => {
                assert!(content.insert(name.to_string(), After::dir()).is_none());
            }
            _ => unreachable!(),
        }
    }

    fn file(&mut self, name: &str, size: usize) {
        match &mut self.after {
            After::Directory(content) => {
                assert!(content.insert(name.to_string(), After::file(size)).is_none());
            }
            _ => unreachable!(),
        }
    }
}

fn solve(after: &After, result: &mut Vec<usize>) -> usize {
    match after {
        After::Directory(content) => {
            let size = content.values().map(|x| solve(x, result)).sum();
            result.push(size);
            size
        }
        After::File(size) => *size,
        After::Invalid => unreachable!(),
    }
}

fn entry(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    assert_eq!(lines.next().unwrap()?, "$ cd /");
    let mut view = View::new();
    for line in lines {
        let line = line?;
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "$" => match words.next().unwrap() {
                "cd" => view.cd(words.next().unwrap()),
                "ls" => (),
                _ => unreachable!(),
            },
            "dir" => view.dir(words.next().unwrap()),
            size => view.file(words.next().unwrap(), size.parse().unwrap()),
        }
        assert!(words.next().is_none());
    }
    while !matches!(view.before, Before::Root) {
        view.cd("..");
    }
    let after = view.after;
    let mut result = Vec::new();
    solve(&after, &mut result);
    let limit = result.iter().max().unwrap() - 40000000;
    let result = result.iter().filter(|&&x| limit <= x).min().unwrap();
    writeln!(output, "{result}")?;
    Ok(())
}

adventofcode::main!(entry("examples/07.txt") == "7991939\n");
