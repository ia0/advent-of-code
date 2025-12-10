use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use z3::ast::Int;
use z3::{Optimize, SatResult, Symbol};

fn trim(data: &str, open: char, close: char) -> &str {
    data.strip_prefix(open).unwrap().strip_suffix(close).unwrap()
}

fn parse(xs: &str, open: char, close: char) -> Vec<usize> {
    trim(xs, open, close).split(',').map(|x| x.parse().unwrap()).collect()
}

fn dist(target: Vec<usize>, edges: &[Vec<usize>]) -> usize {
    let problem = Optimize::new();
    let n = edges.len();
    let consts: Vec<_> = (0 .. n).map(|i| Int::new_const(Symbol::Int(i as u32))).collect();
    for x in &consts {
        problem.assert(&x.ge(0));
    }
    for (j, &target) in target.iter().enumerate() {
        let values: Vec<_> =
            (0 .. n).filter_map(|i| edges[i].contains(&j).then_some(consts[i].clone())).collect();
        problem.assert(&Int::add(&values).eq(target as u32));
    }
    problem.minimize(&Int::add(&consts));
    assert_eq!(problem.check(&[]), SatResult::Sat);
    let model = problem.get_model().unwrap();
    (0 .. n).map(|i| model.eval(&consts[i], true).unwrap().as_i64().unwrap()).sum::<i64>() as usize
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut total = 0;
    for line in BufReader::new(input).lines() {
        let line = line?;
        let mut words = line.split_whitespace();
        let _lights = trim(words.next().unwrap(), '[', ']');
        let mut buttons: Vec<_> = words.collect();
        let joltage = parse(buttons.pop().unwrap(), '{', '}');
        let buttons: Vec<_> = buttons.into_iter().map(|x| parse(x, '(', ')')).collect();
        total += dist(joltage, &buttons);
    }
    writeln!(output, "{total}")?;
    Ok(())
}

adventofcode::main!(solve("examples/10.txt") == "18559\n");
