use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = match File::open("data/02.in") {
        Err(err) => panic!("Couldn't read file : {}", err),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    /* string into vector */
    let mut program: Vec<usize> = contents.split(',').map(|s| s.parse().unwrap()).collect();

    program[1] = 12;
    program[2] = 2;

    for op_num in 0..(program.len() / 4) {
        let pos = op_num * 4;

        let instruction: [usize; 4] = [
            program[pos],
            program[pos + 1],
            program[pos + 2],
            program[pos + 3],
        ];
        match instruction[0] {
            1 => program[instruction[3]] = program[instruction[1]] + program[instruction[2]],
            2 => program[instruction[3]] = program[instruction[1]] * program[instruction[2]],
            99 => break,
            _ => panic!("invalid instruction"),
        }
    }
    println!("{:?}", program);

    Ok(())
}
