use intcode::Computer;

fn main() {
    // 01
    let mut computer = Computer::new(&std::fs::read_to_string("data/02.in").unwrap());

    computer.memory[1] = 12;
    computer.memory[2] = 2;

    while computer.execute().is_ok() {}

    assert_eq!(computer.memory[0], 3_716_250);

    // 02
    loop {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut computer = Computer::new(&std::fs::read_to_string("data/02.in").unwrap());
                computer.memory[1] = noun;
                computer.memory[2] = verb;

                while computer.execute().is_ok() {}
                if computer.memory[0] == 19_690_720 {
                    println!("answer: {}", 100 * noun + verb);
                    return;
                }
            }
        }
    }
}
