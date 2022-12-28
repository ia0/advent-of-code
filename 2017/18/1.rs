use std::io::BufRead;

#[derive(Clone, Copy)]
struct Reg(u8);

impl std::str::FromStr for Reg {
    type Err = ();

    fn from_str(input: &str) -> Result<Reg, ()> {
        if let Ok(x) = input.parse() {
            assert!('a' <= x && x <= 'z');
            Ok(Reg(x as u8 - b'a'))
        } else {
            panic!("Unexpected register {:?}", input);
        }
    }
}

#[derive(Clone, Copy)]
enum Val {
    Lit(i64),
    Reg(Reg),
}

impl std::str::FromStr for Val {
    type Err = ();

    fn from_str(input: &str) -> Result<Val, ()> {
        if let Ok(x) = input.parse() {
            Ok(Val::Lit(x))
        } else {
            Ok(Val::Reg(input.parse()?))
        }
    }
}

#[derive(Clone, Copy)]
enum Instr {
    Snd(Val),
    Set(Reg, Val),
    Add(Reg, Val),
    Mul(Reg, Val),
    Mod(Reg, Val),
    Rcv(Val),
    Jgz(Val, Val),
}

impl std::str::FromStr for Instr {
    type Err = ();

    fn from_str(input: &str) -> Result<Instr, ()> {
        let words: Vec<&str> = input.split_whitespace().collect();
        assert!(words.len() >= 2);
        if words[0] == "snd" {
            assert_eq!(words.len(), 2);
            Ok(Instr::Snd(words[1].parse()?))
        } else if words[0] == "set" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Set(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "add" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Add(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "mul" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Mul(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "mod" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Mod(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "rcv" {
            assert_eq!(words.len(), 2);
            Ok(Instr::Rcv(words[1].parse()?))
        } else if words[0] == "jgz" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Jgz(words[1].parse()?, words[2].parse()?))
        } else {
            panic!("Unexpected input {:?}", input);
        }
    }
}

fn eval(regs: &[i64; 26], val: Val) -> i64 {
    match val {
        Val::Lit(val) => val,
        Val::Reg(reg) => regs[reg.0 as usize],
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut instrs = Vec::new();
    for line in stdin.lock().lines() {
        instrs.push(line.unwrap().parse::<Instr>().unwrap());
    }
    let mut regs = [0; 26];
    let mut pos: i64 = 0;
    let mut freq = None;
    while 0 <= pos && pos < instrs.len() as i64 {
        match instrs[pos as usize] {
            Instr::Snd(val) => freq = Some(eval(&regs, val)),
            Instr::Set(reg, val) => regs[reg.0 as usize] = eval(&regs, val),
            Instr::Add(reg, val) => regs[reg.0 as usize] += eval(&regs, val),
            Instr::Mul(reg, val) => regs[reg.0 as usize] *= eval(&regs, val),
            Instr::Mod(reg, val) => regs[reg.0 as usize] %= eval(&regs, val),
            Instr::Rcv(val) => {
                if eval(&regs, val) != 0 {
                    println!("{}", freq.unwrap());
                    return;
                }
            }
            Instr::Jgz(cond, offset) => {
                if eval(&regs, cond) > 0 {
                    pos += eval(&regs, offset);
                    continue;
                }
            }
        }
        pos += 1;
    }
    panic!();
}
