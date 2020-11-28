use deku::prelude::*;

pub struct Computer {
    pub memory: Vec<usize>,
    pub pc: usize,
}

impl Computer {
    /// initialize computer memory
    pub fn new(data_str: &str) -> Self {
        let memory = data_str.split(',').map(|s| s.parse().unwrap()).collect();
        Self { memory, pc: 0 }
    }

    /// execute next instruction(4 numbers)
    pub fn execute(&mut self) -> Result<(), &'static str> {
        // grab next 4 numbers, parsing into Instruction.
        let instruction: [usize; 4] = [
            self.memory[self.pc],
            self.memory[self.pc + 1],
            self.memory[self.pc + 2],
            self.memory[self.pc + 3],
        ];
        let op_code = OpCode::from_bytes((&[instruction[0] as u8], 0)).unwrap().1;

        // increment pc
        self.pc += 4;

        // execute
        match op_code {
            OpCode::Add => {
                self.memory[instruction[3]] =
                    self.memory[instruction[1]] + self.memory[instruction[2]]
            }
            OpCode::Multiply => {
                self.memory[instruction[3]] =
                    self.memory[instruction[1]] * self.memory[instruction[2]]
            }
            OpCode::Break => return Err("OpCode 99"),
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum OpCode {
    #[deku(id = "1")]
    Add,
    #[deku(id = "2")]
    Multiply,
    #[deku(id = "99")]
    Break,
}
