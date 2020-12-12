use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
pub enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = input.split_ascii_whitespace().collect();
        let arg = input[1].parse().unwrap();
        match input.get(0) {
            Some(&"acc") => Ok(Self::ACC(arg)),
            Some(&"jmp") => Ok(Self::JMP(arg)),
            Some(&"nop") => Ok(Self::NOP(arg)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    HALT,
}

#[derive(Debug)]
pub struct State {
    pc: usize,
    acc: isize,
}

impl State {
    fn new() -> Self {
        Self { pc: 0, acc: 0 }
    }

    fn step(&mut self, prog: &[Instruction]) -> Option<Flag> {
        prog.get(self.pc).map_or(Some(Flag::HALT), |inst| {
            if match inst {
                Instruction::ACC(arg) => {
                    self.acc += arg;
                    true
                }
                Instruction::JMP(arg) => {
                    self.pc = self.pc.wrapping_add(*arg as usize);
                    false
                }
                Instruction::NOP(_) => true,
            } {
                // increment pc by one if true
                self.pc += 1;
            }
            None
        })
    }
}

fn run_to_completion(prog: &[Instruction]) -> (Option<Flag>, State) {
    let mut state = State::new();
    let mut visited: HashSet<usize> = HashSet::new();
    while visited.insert(state.pc) {
        if let Some(flag) = state.step(prog) {
            return (Some(flag), state);
        }
    }
    (None, state)
}

pub fn eight() -> (isize, isize) {
    let tape: Vec<Instruction> = fs::read_to_string("data/08.in")
        .unwrap()
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let part1 = run_to_completion(&tape).1.acc;

    for i in 0..tape.len() {
        let op = match &tape[i] {
            Instruction::JMP(arg) => Instruction::NOP(*arg),
            Instruction::NOP(arg) => Instruction::JMP(*arg),
            _ => continue,
        };
        let mut mod_tape = tape.clone();
        mod_tape[i] = op;

        if let (Some(_), state) = run_to_completion(&mod_tape) {
            return (part1, state.acc);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight() {
        assert_eq!((1179, 1089), eight());
    }
}
