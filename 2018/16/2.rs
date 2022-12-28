use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

const OPCODES: [Opcode; 16] = [
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

struct Instruction {
    x: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn execute(&self, r: &mut [usize; 4]) {
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

struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
}

fn four_numbers(input: &str) -> [usize; 4] {
    let input: Vec<usize> = input
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(input.len(), 4);
    let mut result = [0; 4];
    result.copy_from_slice(&input);
    result
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut samples = Vec::new();
    loop {
        let before = lines.next().unwrap().unwrap();
        if before.is_empty() {
            break;
        }
        let before = four_numbers(&before);
        let instruction = four_numbers(&lines.next().unwrap().unwrap());
        let after = four_numbers(&lines.next().unwrap().unwrap());
        let empty = lines.next().unwrap().unwrap();
        assert!(empty.is_empty());
        assert!(instruction[0] < 16);
        assert!(instruction[1] < 4);
        assert!(instruction[2] < 4);
        assert!(instruction[3] < 4);
        samples.push(Sample { before, instruction, after });
    }
    let mut opcodes = HashMap::new();
    for x in 0 .. 16 {
        opcodes.insert(x, <HashSet<Opcode>>::from_iter(OPCODES.iter().cloned()));
    }
    for sample in samples {
        opcodes.get_mut(&sample.instruction[0]).unwrap().retain(|&x| {
            let instruction = Instruction {
                x,
                a: sample.instruction[1],
                b: sample.instruction[2],
                c: sample.instruction[3],
            };
            let mut registers = sample.before;
            instruction.execute(&mut registers);
            registers == sample.after
        });
    }
    let mut opcode_numbers = HashMap::new();
    while !opcodes.is_empty() {
        opcodes.retain(|&x, opcodes| {
            opcodes.retain(|opcode| !opcode_numbers.contains_key(opcode));
            let done = opcodes.len() == 1;
            if done {
                assert!(opcode_numbers.insert(*opcodes.iter().next().unwrap(), x).is_none());
            }
            !done
        });
    }
    let mut opcodes = Vec::new();
    for i in 0 .. 16 {
        assert_eq!(opcodes.len(), i);
        for (&x, &j) in opcode_numbers.iter() {
            if i == j {
                opcodes.push(x);
                break;
            }
        }
    }
    assert!(lines.next().unwrap().unwrap().is_empty());
    let mut registers = [0; 4];
    for line in lines {
        let instruction = four_numbers(&line.unwrap());
        let instruction = Instruction {
            x: opcodes[instruction[0]],
            a: instruction[1],
            b: instruction[2],
            c: instruction[3],
        };
        instruction.execute(&mut registers);
    }
    println!("{}", registers[0]);
}
