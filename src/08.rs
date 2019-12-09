use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = match File::open("data/08.in") {
        Err(err) => panic!("Couldn't read file : {}", err),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let line = contents
        .as_bytes()
        .chunks_exact(25 * 6)
        .min_by_key(|a| a.iter().filter(|&&x| x == b'0').count())
        .unwrap();

    let num_of_1 = line.iter().filter(|&&b| b == b'1').count();
    let num_of_2 = line.iter().filter(|&&c| c == b'2').count();
    let result = num_of_1 * num_of_2;

    println!("{:?}", result);

    Ok(())
}
