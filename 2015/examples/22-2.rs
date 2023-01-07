#![feature(int_roundings)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, BufReader, Lines, Read, Write};

use anyhow::Result;

fn parse(lines: &mut Lines<BufReader<impl Read>>, prefix: &str) -> usize {
    let line = lines.next().unwrap().unwrap();
    let (a, b) = line.split_once(": ").unwrap();
    assert_eq!(a, prefix);
    b.parse().unwrap()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Turn {
    Wizard,
    Boss,
}

impl Turn {
    fn tick(&mut self) -> Turn {
        let result = *self;
        *self = match result {
            Turn::Wizard => Turn::Boss,
            Turn::Boss => Turn::Wizard,
        };
        result
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Life(usize);

impl Life {
    fn dead(&self) -> bool {
        self.0 == 0
    }

    fn remove(&mut self, damage: usize) {
        self.0 = self.0.saturating_sub(damage);
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Mana(usize);

impl Mana {
    fn consume(&mut self, cost: usize) -> bool {
        let result = cost <= self.0;
        if result {
            self.0 -= cost;
        }
        result
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Effect(usize);

impl Effect {
    fn active(&self) -> bool {
        self.0 > 0
    }

    fn tick(&mut self) -> bool {
        let result = self.0 > 0;
        if result {
            self.0 -= 1;
        }
        result
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    boss: Reverse<Life>,
    shield: Effect,
    poison: Effect,
    recharge: Effect,
    mana: Mana,
    wizard: Life,
    turn: Turn,
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let boss = parse(&mut lines, "Hit Points");
    let damage = parse(&mut lines, "Damage");
    assert!(lines.next().is_none());
    let mut todo = BinaryHeap::new();
    let mut visited = HashSet::new();
    todo.push((
        Reverse(0),
        State {
            boss: Reverse(Life(boss)),
            shield: Effect(0),
            poison: Effect(0),
            recharge: Effect(0),
            mana: Mana(500),
            wizard: Life(50),
            turn: Turn::Wizard,
        },
    ));
    while let Some((Reverse(spent), mut cur)) = todo.pop() {
        if !visited.insert(cur) {
            continue;
        }
        // Hard mode.
        if matches!(cur.turn, Turn::Wizard) {
            cur.wizard.remove(1);
            if cur.wizard.dead() {
                continue;
            }
        }
        // Apply effects.
        cur.shield.tick();
        if cur.poison.tick() {
            cur.boss.0.remove(3);
        }
        if cur.recharge.tick() {
            cur.mana.0 += 101;
        }
        // Check if boss is dead.
        if cur.boss.0.dead() {
            writeln!(output, "{spent}")?;
            break;
        }
        // Boss turn.
        if matches!(cur.turn.tick(), Turn::Boss) {
            let mut damage = damage;
            if cur.shield.active() {
                damage = std::cmp::max(1, damage.saturating_sub(7));
            }
            cur.wizard.remove(damage);
            if !cur.wizard.dead() {
                todo.push((Reverse(spent), cur));
            }
            continue;
        }
        // Magic missile.
        let mut next = cur;
        if next.mana.consume(53) {
            next.boss.0.remove(4);
            todo.push((Reverse(spent + 53), next));
        }
        // Drain.
        let mut next = cur;
        if next.mana.consume(73) {
            next.boss.0.remove(2);
            next.wizard.0 += 2;
            todo.push((Reverse(spent + 73), next));
        }
        // Shield.
        let mut next = cur;
        if !cur.shield.active() && next.mana.consume(113) {
            next.shield.0 = 6;
            todo.push((Reverse(spent + 113), next));
        }
        // Poison.
        let mut next = cur;
        if !cur.poison.active() && next.mana.consume(173) {
            next.poison.0 = 6;
            todo.push((Reverse(spent + 173), next));
        }
        // Recharge.
        let mut next = cur;
        if !cur.recharge.active() && next.mana.consume(229) {
            next.recharge.0 = 5;
            todo.push((Reverse(spent + 229), next));
        }
    }
    Ok(())
}

adventofcode::main!(solve("examples/22.txt") == "1937\n");
