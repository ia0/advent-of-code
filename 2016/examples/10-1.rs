use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::{bail, Result};
use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Slot {
    Bot(usize),
    Output(usize),
}

#[derive(Copy, Clone)]
struct Rule {
    low: Slot,
    high: Slot,
}

impl Slot {
    fn new(slot: &str, id: &str) -> Result<Self> {
        let id = id.parse()?;
        Ok(match slot {
            "bot" => Slot::Bot(id),
            "output" => Slot::Output(id),
            _ => bail!("bad slot {slot}"),
        })
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut slots = HashMap::<Slot, Vec<usize>>::new();
    let mut rules = HashMap::new();
    let init = Regex::new("^value (.*) goes to bot (.*)$")?;
    let rule = Regex::new("^bot (.*) gives low to (.*) (.*) and high to (.*) (.*)$")?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        if let Some(captures) = init.captures(&line) {
            let chip = captures[1].parse::<usize>()?;
            let bot = captures[2].parse::<usize>()?;
            slots.entry(Slot::Bot(bot)).or_default().push(chip);
        } else if let Some(captures) = rule.captures(&line) {
            let bot = captures[1].parse::<usize>()?;
            let low = Slot::new(&captures[2], &captures[3])?;
            let high = Slot::new(&captures[4], &captures[5])?;
            assert!(rules.insert(bot, Rule { low, high }).is_none());
        } else {
            unreachable!();
        }
    }
    loop {
        assert!(slots.values().all(|x| x.len() <= 2));
        let bot = slots
            .iter()
            .find_map(|(slot, chips)| match slot {
                Slot::Bot(x) if chips.len() == 2 => Some(*x),
                _ => None,
            })
            .unwrap();
        let mut chips = slots.remove(&Slot::Bot(bot)).unwrap();
        chips.sort();
        if chips == [17, 61] {
            writeln!(output, "{bot}")?;
            break;
        }
        let Rule { low, high } = rules[&bot];
        slots.entry(low).or_default().push(chips[0]);
        slots.entry(high).or_default().push(chips[1]);
    }
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "73\n");
