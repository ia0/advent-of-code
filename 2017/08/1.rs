use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug)]
enum Operator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl std::str::FromStr for Operator {
    type Err = ();

    fn from_str(input: &str) -> Result<Operator, ()> {
        Ok(if input == "==" {
            Operator::Eq
        } else if input == "!=" {
            Operator::Ne
        } else if input == "<" {
            Operator::Lt
        } else if input == "<=" {
            Operator::Le
        } else if input == ">" {
            Operator::Gt
        } else {
            assert_eq!(input, ">=");
            Operator::Ge
        })
    }
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: Operator,
    value: i64,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    offset: i64,
    condition: Condition,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, ()> {
        let words: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(words.len(), 7);
        let register = words[0].to_owned();
        let mut offset: i64 = words[2].parse().unwrap();
        if words[1] == "dec" {
            offset = -offset;
        } else {
            assert_eq!(words[1], "inc");
        }
        assert_eq!(words[3], "if");
        let condition = {
            let register = words[4].to_owned();
            let operator = words[5].parse().unwrap();
            let value = words[6].parse().unwrap();
            Condition { register, operator, value }
        };
        Ok(Instruction { register, offset, condition })
    }
}

fn eval(registers: &HashMap<String, i64>, condition: Condition) -> bool {
    let register = *registers.get(condition.register.as_str()).unwrap_or(&0);
    let value = condition.value;
    match condition.operator {
        Operator::Eq => register == value,
        Operator::Ne => register != value,
        Operator::Lt => register < value,
        Operator::Le => register <= value,
        Operator::Gt => register > value,
        Operator::Ge => register >= value,
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut registers = HashMap::new();
    for line in stdin.lock().lines() {
        let instruction: Instruction = line.unwrap().parse().unwrap();
        if eval(&registers, instruction.condition) {
            *registers.entry(instruction.register).or_insert(0) += instruction
                .offset;
        }
    }
    println!("{}", registers.values().max().unwrap());
}
