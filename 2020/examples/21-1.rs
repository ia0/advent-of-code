use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Relation {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Relation {
    fn extend(&mut self, other: &Relation) {
        self.ingredients.extend(other.ingredients.iter().cloned());
        self.allergens.extend(other.allergens.iter().cloned());
    }

    fn from_solution(solution: &HashMap<String, String>) -> Relation {
        Relation {
            ingredients: solution.values().cloned().collect(),
            allergens: solution.keys().cloned().collect(),
        }
    }

    fn difference(&self, other: &Relation) -> Relation {
        Relation {
            ingredients: self.ingredients.difference(&other.ingredients).cloned().collect(),
            allergens: self.allergens.difference(&other.allergens).cloned().collect(),
        }
    }
}

fn solve(data: &[Relation], all: &Relation, solution: &mut HashMap<String, String>) -> bool {
    let resolved = Relation::from_solution(solution);
    assert_eq!(resolved.ingredients.len(), resolved.allergens.len());
    let unresolved = all.difference(&resolved);
    let (allergen, ingredients) = match unresolved
        .allergens
        .iter()
        .map(|x| {
            (
                x,
                data.iter().filter(|y| y.allergens.contains(x)).fold(
                    unresolved.ingredients.clone(),
                    |mut a, y| {
                        a.retain(|z| y.ingredients.contains(z));
                        a
                    },
                ),
            )
        })
        .min_by_key(|(_, x)| x.len())
    {
        None => return true,
        Some(x) => x,
    };
    for ingredient in ingredients {
        assert!(solution.insert(allergen.clone(), ingredient).is_none());
        if solve(data, all, solution) {
            return true;
        }
        assert!(solution.remove(allergen).is_some());
    }
    false
}

fn main() {
    let input = File::open("examples/21.txt").unwrap();
    let mut data = Vec::new();
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        let mut relation = Relation::default();
        for word in line.split_whitespace() {
            if word == "(contains" {
            } else if let Some(word) = word.strip_suffix(',') {
                assert!(relation.allergens.insert(word.to_string()));
            } else if let Some(word) = word.strip_suffix(')') {
                assert!(relation.allergens.insert(word.to_string()));
            } else {
                assert!(relation.ingredients.insert(word.to_string()));
            }
        }
        data.push(relation);
    }
    let all = data.iter().fold(Relation::default(), |mut all, x| {
        all.extend(x);
        all
    });
    let mut solution = HashMap::new();
    assert!(solve(&data, &all, &mut solution));
    let domain = Relation::from_solution(&solution);
    let unmatched = all.difference(&domain);
    assert_eq!(unmatched.allergens.len(), 0);
    println!(
        "{}",
        data.iter()
            .map(|x| x.ingredients.intersection(&unmatched.ingredients).count())
            .sum::<usize>()
    );
}
