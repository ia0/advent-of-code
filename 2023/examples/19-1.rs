use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
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

struct Part {
    ratings: [i64; 4],
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

impl FromStr for Part {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let input = input.strip_prefix('{').context("no {")?;
        let input = input.strip_suffix('}').context("no }")?;
        let mut ratings = [0; 4];
        for (i, rating) in input.split(',').enumerate() {
            let (category, rating) = rating.split_once('=').context("no =")?;
            ensure!(CATEGORIES.get(i) == Some(&category));
            ratings[i] = rating.parse()?;
        }
        Ok(Part { ratings })
    }
}

impl Problem {
    fn accept(&self, part: &Part) -> bool {
        let mut name = "in".to_string();
        loop {
            match self.workflows[&name].execute(part) {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Workflow(x) => name = x,
            }
        }
    }
}

impl Workflow {
    fn execute(&self, part: &Part) -> Action {
        for rule in &self.rules {
            if rule.condition.matches(part) {
                return rule.action.clone();
            }
        }
        self.default.clone()
    }
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        let rating = part.ratings[self.variable];
        if self.reversed {
            self.threshold < rating
        } else {
            rating < self.threshold
        }
    }
}

impl Part {
    fn total(&self) -> i64 {
        self.ratings.iter().sum()
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let mut problem = Problem::default();
    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (name, rules) = line.split_once('{').context("no {")?;
        let rules = rules.strip_suffix('}').context("no }")?;
        let workflow = rules.parse()?;
        ensure!(problem.workflows.insert(name.to_string(), workflow).is_none());
    }
    let mut total = 0;
    for line in lines {
        let part = line?.parse::<Part>()?;
        if problem.accept(&part) {
            total += part.total();
        }
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/19.txt") == "342650\n");
