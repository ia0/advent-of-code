#![feature(int_roundings)]

use std::io::{BufRead, BufReader, Lines, Read, Write};

use anyhow::Result;

struct Stats {
    life: usize,
    damage: usize,
    armor: usize,
}

fn parse(lines: &mut Lines<BufReader<impl Read>>, prefix: &str) -> usize {
    let line = lines.next().unwrap().unwrap();
    let (a, b) = line.split_once(": ").unwrap();
    assert_eq!(a, prefix);
    b.parse().unwrap()
}

fn win(me: &Stats, boss: &Stats) -> bool {
    let boss_hit = std::cmp::max(1, boss.damage.saturating_sub(me.armor));
    let me_hit = std::cmp::max(1, me.damage.saturating_sub(boss.armor));
    boss.life.div_ceil(me_hit) <= me.life.div_ceil(boss_hit)
}

#[derive(Copy, Clone)]
struct Equipment {
    cost: usize,
    damage: usize,
    armor: usize,
}

const WEAPONS: [Equipment; 5] = [
    Equipment { cost: 8, damage: 4, armor: 0 },
    Equipment { cost: 10, damage: 5, armor: 0 },
    Equipment { cost: 25, damage: 6, armor: 0 },
    Equipment { cost: 40, damage: 7, armor: 0 },
    Equipment { cost: 74, damage: 8, armor: 0 },
];

const ARMORS: [Equipment; 5] = [
    Equipment { cost: 13, damage: 0, armor: 1 },
    Equipment { cost: 31, damage: 0, armor: 2 },
    Equipment { cost: 53, damage: 0, armor: 3 },
    Equipment { cost: 75, damage: 0, armor: 4 },
    Equipment { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Equipment; 6] = [
    Equipment { cost: 25, damage: 1, armor: 0 },
    Equipment { cost: 50, damage: 2, armor: 0 },
    Equipment { cost: 100, damage: 3, armor: 0 },
    Equipment { cost: 20, damage: 0, armor: 1 },
    Equipment { cost: 40, damage: 0, armor: 2 },
    Equipment { cost: 80, damage: 0, armor: 3 },
];

fn weapon(boss: &Stats) -> usize {
    let mut best = usize::MAX;
    let mut equipments = Vec::new();
    for weapon in WEAPONS {
        equipments.push(weapon);
        armor(boss, &mut best, &mut equipments);
        equipments.pop();
    }
    best
}

fn armor(boss: &Stats, best: &mut usize, equipments: &mut Vec<Equipment>) {
    ring(boss, best, equipments, None);
    for armor in ARMORS {
        equipments.push(armor);
        ring(boss, best, equipments, None);
        equipments.pop();
    }
}

fn ring(boss: &Stats, best: &mut usize, equipments: &mut Vec<Equipment>, other: Option<usize>) {
    if other.is_none() {
        ring(boss, best, equipments, Some(6));
    } else {
        equip(boss, best, equipments);
    }
    for (this, &ring_) in RINGS.iter().enumerate() {
        if Some(this) == other {
            continue;
        }
        equipments.push(ring_);
        if other.is_none() {
            ring(boss, best, equipments, Some(this));
        } else {
            equip(boss, best, equipments);
        }
        equipments.pop();
    }
}

fn equip(boss: &Stats, best: &mut usize, equipments: &[Equipment]) {
    let mut me = Stats { life: 100, damage: 0, armor: 0 };
    let mut cost = 0;
    for &equipment in equipments {
        cost += equipment.cost;
        me.damage += equipment.damage;
        me.armor += equipment.armor;
    }
    if win(&me, boss) {
        *best = std::cmp::min(*best, cost);
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let boss = Stats {
        life: parse(&mut lines, "Hit Points"),
        damage: parse(&mut lines, "Damage"),
        armor: parse(&mut lines, "Armor"),
    };
    writeln!(output, "{}", weapon(&boss))?;
    Ok(())
}

adventofcode::main!(solve("examples/21.txt") == "121\n");
