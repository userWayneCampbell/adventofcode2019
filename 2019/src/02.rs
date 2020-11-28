use intcode::{Computer, OpCode};

fn main() {
    let mut computer = Computer::new(&std::fs::read_to_string("data/02.in").unwrap());

    computer.memory[1] = 12;
    computer.memory[2] = 2;

    while computer.execute().is_ok() {}

    assert_eq!(computer.memory[0], 3716250);
}
