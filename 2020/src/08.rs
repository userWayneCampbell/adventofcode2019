use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = input.split_ascii_whitespace().collect();
        let arg = input[1].parse().unwrap();
        match input[0] {
            "acc" => Ok(Self::ACC(arg)),
            "jmp" => Ok(Self::JMP(arg)),
            "nop" => Ok(Self::NOP(arg)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct State {
    pc: usize,
    acc: isize,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
    HALT,
}

impl State {
    fn new() -> Self {
        Self { pc: 0, acc: 0 }
    }

    fn step(&mut self, prog: &Vec<Instruction>) -> Option<Flag> {
        if let Some(inst) = prog.get(self.pc) {
            if match inst {
                Instruction::ACC(arg) => {
                    self.acc += arg;
                    true
                }
                Instruction::JMP(arg) => {
                    self.pc = (self.pc as isize + arg) as usize;
                    false
                }
                Instruction::NOP(_) => true,
            } {
                // increment pc by one if true
                self.pc += 1;
            }
            None
        } else {
            Some(Flag::HALT)
        }
    }
}

fn run_to_completion(prog: &Vec<Instruction>) -> (Option<Flag>, State) {
    let mut state = State::new();
    let mut visited: HashSet<usize> = HashSet::new();
    while visited.insert(state.pc) {
        if let Some(flag) = state.step(prog) {
            return (Some(flag), state);
        }
    }
    (None, state)
}

fn main() {
    let tape: Vec<Instruction> = fs::read_to_string("data/08.in")
        .unwrap()
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let part1 = run_to_completion(&tape).1.acc;
    assert_eq!(part1, 1179);
    println!("Part 1: {}", part1);

    for i in 0..tape.len() {
        let op = match &tape[i] {
            Instruction::JMP(arg) => Instruction::NOP(*arg),
            Instruction::NOP(arg) => Instruction::JMP(*arg),
            _ => continue,
        };
        let mut mod_tape = tape.clone();
        mod_tape[i] = op;

        if let (Some(_), state) = run_to_completion(&mod_tape) {
            assert_eq!(1089, state.acc);
            println!("Part 2: {}", state.acc);
            break;
        }
    }
}
