use std::io::BufRead;

#[derive(Clone, Copy)]
struct Reg(u8);

impl std::str::FromStr for Reg {
    type Err = ();

    fn from_str(input: &str) -> Result<Reg, ()> {
        if let Ok(x) = input.parse() {
            assert!('a' <= x && x <= 'h');
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
    Set(Reg, Val),
    Sub(Reg, Val),
    Mul(Reg, Val),
    Jnz(Val, Val),
}

impl std::str::FromStr for Instr {
    type Err = ();

    fn from_str(input: &str) -> Result<Instr, ()> {
        let words: Vec<&str> = input.split_whitespace().collect();
        assert!(words.len() >= 2);
        if words[0] == "set" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Set(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "sub" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Sub(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "mul" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Mul(words[1].parse()?, words[2].parse()?))
        } else if words[0] == "jnz" {
            assert_eq!(words.len(), 3);
            Ok(Instr::Jnz(words[1].parse()?, words[2].parse()?))
        } else {
            panic!("Unexpected input {:?}", input);
        }
    }
}

fn eval(regs: &[i64; 8], val: Val) -> i64 {
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
    let mut regs = [0; 8];
    let mut pos: i64 = 0;
    let mut count = 0;
    while 0 <= pos && pos < instrs.len() as i64 {
        match instrs[pos as usize] {
            Instr::Set(reg, val) => regs[reg.0 as usize] = eval(&regs, val),
            Instr::Sub(reg, val) => regs[reg.0 as usize] -= eval(&regs, val),
            Instr::Mul(reg, val) => {
                count += 1;
                regs[reg.0 as usize] *= eval(&regs, val)
            }
            Instr::Jnz(cond, offset) => {
                if eval(&regs, cond) != 0 {
                    pos += eval(&regs, offset);
                    continue;
                }
            }
        }
        pos += 1;
    }
    println!("{}", count);
}
