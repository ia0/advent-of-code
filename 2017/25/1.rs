use std::collections::HashMap;
use std::io::BufRead;

fn extract<'a>(line: &'a str, prefix: &str, suffix: &str) -> &'a str {
    assert!(line.starts_with(prefix));
    assert!(line.ends_with(suffix));
    line.split_at(line.len() - suffix.len()).0.split_at(prefix.len()).1
}

fn extract_state(line: String, prefix: &str, suffix: &str) -> usize {
    let middle = extract(&line, prefix, suffix).as_bytes();
    assert_eq!(middle.len(), 1);
    (middle[0] - b'A') as usize
}

fn extract_value(line: String, prefix: &str, suffix: &str) -> usize {
    let middle = extract(&line, prefix, suffix).as_bytes();
    assert_eq!(middle.len(), 1);
    (middle[0] - b'0') as usize
}

fn extract_dir(line: String, prefix: &str, suffix: &str) -> usize {
    let middle = extract(&line, prefix, suffix);
    if middle == "left" {
        0
    } else {
        assert_eq!(middle, "right");
        1
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut state = extract_state(lines.next().unwrap().unwrap(),
                                  "Begin in state ", ".");
    let steps: usize = extract(&lines.next().unwrap().unwrap(),
                               "Perform a diagnostic checksum after ",
                               " steps.").parse().unwrap();
    let mut rules = Vec::new();
    loop {
        if let Some(x) = lines.next() {
            assert!(x.unwrap().is_empty());
        } else {
            break;
        }
        assert_eq!(rules.len(),
                   2 * extract_state(lines.next().unwrap().unwrap(),
                                     "In state ", ":"));
        for i in 0 .. 2 {
            assert_eq!(lines.next().unwrap().unwrap(),
                       format!("  If the current value is {}:", i));
            let value = extract_value(lines.next().unwrap().unwrap(),
                                      "    - Write the value ", ".");
            let dir = extract_dir(lines.next().unwrap().unwrap(),
                                  "    - Move one slot to the ", ".");
            let state = extract_state(lines.next().unwrap().unwrap(),
                                      "    - Continue with state ", ".");
            rules.push(state << 2 | dir << 1 | value);
        }
    }
    let mut tape = HashMap::new();
    let mut pos: i64 = 0;
    for _ in 0 .. steps {
        let cell = tape.entry(pos).or_insert(0);
        let rule = rules[2 * state + *cell];
        *cell = rule & 1;
        if rule & 2 != 0 {
            pos += 1;
        } else {
            pos -= 1;
        }
        state = rule >> 2;
    }
    println!("{}", tape.values().sum::<usize>());
}
