use std::io::BufRead;

#[derive(Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use Opcode::*;

struct Instruction {
    x: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn execute(&self, r: &mut [usize; 6]) {
        match self.x {
            Addr => r[self.c] = r[self.a] + r[self.b],
            Addi => r[self.c] = r[self.a] + self.b,
            Mulr => r[self.c] = r[self.a] * r[self.b],
            Muli => r[self.c] = r[self.a] * self.b,
            Banr => r[self.c] = r[self.a] & r[self.b],
            Bani => r[self.c] = r[self.a] & self.b,
            Borr => r[self.c] = r[self.a] | r[self.b],
            Bori => r[self.c] = r[self.a] | self.b,
            Setr => r[self.c] = r[self.a],
            Seti => r[self.c] = self.a,
            Gtir => r[self.c] = (self.a > r[self.b]) as usize,
            Gtri => r[self.c] = (r[self.a] > self.b) as usize,
            Gtrr => r[self.c] = (r[self.a] > r[self.b]) as usize,
            Eqir => r[self.c] = (self.a == r[self.b]) as usize,
            Eqri => r[self.c] = (r[self.a] == self.b) as usize,
            Eqrr => r[self.c] = (r[self.a] == r[self.b]) as usize,
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let ip = (lines.next().unwrap().unwrap().as_bytes()[4] - b'0') as usize;
    let mut program = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(words.len(), 4);
        let x = if words[0] == "addr" {
            Addr
        } else if words[0] == "addi" {
            Addi
        } else if words[0] == "mulr" {
            Mulr
        } else if words[0] == "muli" {
            Muli
        } else if words[0] == "banr" {
            Banr
        } else if words[0] == "bani" {
            Bani
        } else if words[0] == "borr" {
            Borr
        } else if words[0] == "bori" {
            Bori
        } else if words[0] == "setr" {
            Setr
        } else if words[0] == "seti" {
            Seti
        } else if words[0] == "gtir" {
            Gtir
        } else if words[0] == "gtri" {
            Gtri
        } else if words[0] == "gtrr" {
            Gtrr
        } else if words[0] == "eqir" {
            Eqir
        } else if words[0] == "eqri" {
            Eqri
        } else {
            assert_eq!(words[0], "eqrr");
            Eqrr
        };
        let a = words[1].parse().unwrap();
        let b = words[2].parse().unwrap();
        let c = words[3].parse().unwrap();
        program.push(Instruction { x, a, b, c });
    }
    let mut i = 0;
    let mut r = [0; 6];
    while i < program.len() {
        r[ip] = i;
        if i == 28 {
            println!("{}", r[5]);
            return;
        }
        program[i].execute(&mut r);
        i = r[ip];
        i += 1;
    }
}
