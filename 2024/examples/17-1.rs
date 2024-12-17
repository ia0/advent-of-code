use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

struct Computer {
    registers: [i64; 3],
    position: i64,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn step(&mut self) -> bool {
        let Some(opcode) = self.program.get(self.position as usize) else { return false };
        let operand = self.program[self.position as usize + 1];
        self.position += 2;
        match opcode {
            0 => self.registers[0] >>= self.combo(operand),
            1 => self.registers[1] ^= operand as i64,
            2 => self.registers[1] = self.combo(operand) & 7,
            3 if self.registers[0] == 0 => (),
            3 => self.position = operand as i64,
            4 => self.registers[1] ^= self.registers[2],
            5 => self.output.push((self.combo(operand) & 7) as u8),
            6 => self.registers[1] = self.registers[0] >> self.combo(operand),
            7 => self.registers[2] = self.registers[0] >> self.combo(operand),
            _ => unreachable!(),
        }
        true
    }

    fn combo(&self, operand: u8) -> i64 {
        match operand {
            0 ..= 3 => operand as i64,
            4 ..= 6 => self.registers[(operand - 4) as usize],
            _ => unreachable!(),
        }
    }
}

fn solve(input: impl Read, mut output: impl Write) -> Result<()> {
    let mut lines = BufReader::new(input).lines();
    let registers = std::array::from_fn(|_| {
        lines.next().unwrap().unwrap().split_once(": ").unwrap().1.parse().unwrap()
    });
    assert!(lines.next().unwrap()?.is_empty());
    let program = lines.next().unwrap()?;
    let program = program.split_once(": ").unwrap().1;
    let program = program.split(",").map(|x| x.parse().unwrap()).collect();
    assert!(lines.next().is_none());
    let mut computer = Computer { registers, position: 0, program, output: Vec::new() };
    while computer.step() {}
    let result = computer.output.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(",");
    writeln!(output, "{result}")?;
    Ok(())
}

adventofcode::main!(solve("examples/17.txt") == "6,5,4,7,1,6,0,3,1\n");
