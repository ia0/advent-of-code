use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

fn next(xs: &mut [i64]) -> bool {
    let i = xs.iter().position(|&x| x > 0).unwrap();
    if i == 0 {
        xs[0] -= 1;
        xs[1] += 1;
        return false;
    }
    if i == xs.len() - 1 {
        assert_eq!(xs[i], 100);
        return true;
    }
    xs[i] = 0;
    xs[i + 1] += 1;
    xs[0] = 100 - xs[1 ..].iter().sum::<i64>();
    false
}

fn score(ingredients: &[[i64; 4]], teaspoons: &[i64]) -> i64 {
    let mut r = [0; 4];
    for (xs, k) in ingredients.iter().zip(teaspoons.iter()) {
        for i in 0 .. 4 {
            r[i] += k * xs[i];
        }
    }
    r.into_iter().map(|x| std::cmp::max(0, x)).product()
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut ingredients = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line?;
        let (_, line) = line.split_once(": ").unwrap();
        let mut words = line
            .split(", ")
            .map(|x| x.split_once(' ').unwrap().1.parse())
            .collect::<Result<Vec<i64>, _>>()?;
        assert_eq!(words.len(), 5);
        words.pop();
        ingredients.push(<[i64; 4]>::try_from(words).unwrap());
    }
    let mut teaspoons = vec![0; ingredients.len()];
    teaspoons[0] = 100;
    let mut best = 0;
    loop {
        best = std::cmp::max(best, score(&ingredients, &teaspoons));
        if next(&mut teaspoons) {
            break;
        }
    }
    writeln!(output, "{best}")?;
    Ok(())
}

adventofcode::main!(solve("examples/15.txt") == "222870\n");
