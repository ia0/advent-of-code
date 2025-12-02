use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use regex::Regex;

struct Blueprint {
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl Blueprint {
    fn build_ore(&self, state: &State) -> Option<State> {
        let time = state.ore.time(self.ore)?;
        if state.minutes <= time {
            return None;
        }
        let mut state = state.clone();
        state.update(time + 1);
        state.ore.count -= self.ore;
        state.ore.robot += 1;
        Some(state)
    }

    fn build_clay(&self, state: &State) -> Option<State> {
        let time = state.ore.time(self.clay)?;
        if state.minutes <= time {
            return None;
        }
        let mut state = state.clone();
        state.update(time + 1);
        state.ore.count -= self.clay;
        state.clay.robot += 1;
        Some(state)
    }

    fn build_obsidian(&self, state: &State) -> Option<State> {
        let time = std::cmp::max(
            state.ore.time(self.obsidian.0).unwrap(),
            state.clay.time(self.obsidian.1)?,
        );
        if state.minutes <= time {
            return None;
        }
        let mut state = state.clone();
        state.update(time + 1);
        state.ore.count -= self.obsidian.0;
        state.clay.count -= self.obsidian.1;
        state.obsidian.robot += 1;
        Some(state)
    }

    fn build_geode(&self, state: &State) -> Option<State> {
        let time = std::cmp::max(
            state.ore.time(self.geode.0).unwrap(),
            state.obsidian.time(self.geode.1)?,
        );
        if state.minutes <= time {
            return None;
        }
        let mut state = state.clone();
        state.update(time + 1);
        state.ore.count -= self.geode.0;
        state.obsidian.count -= self.geode.1;
        state.geode.robot += 1;
        Some(state)
    }
}

#[derive(Default, Copy, Clone)]
struct Resource {
    count: usize,
    robot: usize,
}

impl Resource {
    fn time(&self, cost: usize) -> Option<usize> {
        let needed = cost.saturating_sub(self.count);
        if needed > 0 && self.robot == 0 {
            None
        } else {
            Some(needed.div_ceil(self.robot))
        }
    }

    fn update(&mut self, time: usize) {
        self.count += time * self.robot;
    }
}

#[derive(Default, Clone)]
struct State {
    minutes: usize,
    geode: Resource,
    obsidian: Resource,
    clay: Resource,
    ore: Resource,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_geode().cmp(&other.min_geode())
    }
}

impl State {
    fn min_geode(&self) -> usize {
        self.geode.count + self.minutes * self.geode.robot
    }

    fn max_geode(&self) -> usize {
        self.min_geode() + self.minutes * self.minutes.saturating_sub(1) / 2
    }

    fn update(&mut self, time: usize) {
        self.minutes -= time;
        self.geode.update(time);
        self.obsidian.update(time);
        self.clay.update(time);
        self.ore.update(time);
    }
}

fn solve(blueprint: &Blueprint) -> usize {
    let mut todo = BinaryHeap::new();
    todo.push(State { minutes: 32, ore: Resource { count: 0, robot: 1 }, ..State::default() });
    let mut best = 0;
    while let Some(cur) = todo.pop() {
        if cur.max_geode() <= best {
            continue;
        }
        for next in [
            blueprint.build_geode(&cur),
            blueprint.build_obsidian(&cur),
            blueprint.build_clay(&cur),
            blueprint.build_ore(&cur),
        ]
        .into_iter()
        .flatten()
        {
            todo.push(next);
        }
        best = std::cmp::max(best, cur.min_geode());
    }
    best
}

fn entry(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut blueprints = Vec::new();
    let regex = Regex::new(
        "Blueprint (.*): Each ore robot costs (.*) ore. Each clay robot costs (.*) ore. Each \
         obsidian robot costs (.*) ore and (.*) clay. Each geode robot costs (.*) ore and (.*) \
         obsidian.",
    )?;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let captures = regex.captures(&line).unwrap();
        let ore = captures[2].parse()?;
        let clay = captures[3].parse()?;
        let obsidian = (captures[4].parse()?, captures[5].parse()?);
        let geode = (captures[6].parse()?, captures[7].parse()?);
        blueprints.push(Blueprint { ore, clay, obsidian, geode });
        assert_eq!(captures[1].parse::<usize>()?, blueprints.len());
    }
    writeln!(output, "{}", blueprints.iter().take(3).map(solve).product::<usize>())?;
    Ok(())
}

adventofcode::main!(entry("examples/19.txt") == "88160\n");
