use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Range;
use std::str::FromStr;

use anyhow::{ensure, Context, Error, Result};

#[derive(Default)]
struct Problem {
    workflows: HashMap<String, Workflow>,
}

struct Workflow {
    rules: Vec<Rule>,
    default: Action,
}

struct Rule {
    condition: Condition,
    action: Action,
}

struct Condition {
    variable: usize,
    threshold: i64,
    reversed: bool, // if reversed { t < v } else { v < t }
}

#[derive(Clone)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug, Default, Clone)]
struct Part {
    ratings: [Range<i64>; 4],
}

const CATEGORIES: [&str; 4] = ["x", "m", "a", "s"];

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let rules = input.split(',').collect::<Vec<_>>();
        let (default, rules) = rules.split_last().context("empty rules")?;
        let rules = rules.iter().map(|x| x.parse()).collect::<Result<Vec<_>>>()?;
        let default = default.parse()?;
        Ok(Workflow { rules, default })
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (condition, action) = input.split_once(':').context("no :")?;
        let condition = condition.parse()?;
        let action = action.parse()?;
        Ok(Rule { condition, action })
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let (variable, threshold, reversed) = if let Some((v, t)) = input.split_once('<') {
            (v, t, false)
        } else {
            let (v, t) =
                input.split_once('>').with_context(|| format!("bad condition {input:?}"))?;
            (v, t, true)
        };
        let variable =
            CATEGORIES.iter().position(|&x| x == variable).context("invalid variable")?;
        let threshold = threshold.parse()?;
        Ok(Condition { variable, threshold, reversed })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        Ok(match input {
            "A" => Action::Accept,
            "R" => Action::Reject,
            x => Action::Workflow(x.to_string()),
        })
    }
}

impl Problem {
    fn count(&self, name: &str, mut part: Part) -> i64 {
        let mut total = 0;
        let workflow = &self.workflows[name];
        for rule in &workflow.rules {
            let matches = rule.condition.split(&mut part);
            total += match &rule.action {
                Action::Accept => matches.count(),
                Action::Reject => 0,
                Action::Workflow(name) => self.count(name, matches.clone()),
            }
        }
        total += match &workflow.default {
            Action::Accept => part.count(),
            Action::Reject => 0,
            Action::Workflow(name) => self.count(name, part),
        };
        total
    }
}

impl Condition {
    fn split(&self, part: &mut Part) -> Part {
        let mut matches = part.clone();
        let no = &mut part.ratings[self.variable];
        let yes = &mut matches.ratings[self.variable];
        if self.reversed {
            yes.start = std::cmp::max(yes.start, self.threshold + 1);
            no.end = std::cmp::min(no.end, self.threshold + 1);
        } else {
            yes.end = std::cmp::min(yes.end, self.threshold);
            no.start = std::cmp::max(no.start, self.threshold);
        }
        matches
    }
}

impl Part {
    fn count(&self) -> i64 {
        self.ratings.iter().map(|x| x.end - x.start).product()
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut problem = Problem::default();
    for line in BufReader::new(input).lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (name, rules) = line.split_once('{').context("no {")?;
        let rules = rules.strip_suffix('}').context("no }")?;
        let workflow = rules.parse()?;
        ensure!(problem.workflows.insert(name.to_string(), workflow).is_none());
    }
    let part = Part { ratings: std::array::from_fn(|_| 1 .. 4001) };
    let total = problem.count("in", part);
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "130303473508222\n");
