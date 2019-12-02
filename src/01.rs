use std::fs::File;
use std::io::Read;

fn fuel_from_mass(m: i32) -> i32 {
    std::cmp::max(m / 3 - 2, 0)
}

fn main() -> std::io::Result<()> {
    let mut file = match File::open("data/01.in") {
        Err(err) => panic!("Couldn't read file : {}", err),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    /* string into vector */
    let masses: Vec<i32> = contents.lines().flat_map(|s| s.trim().parse()).collect();

    /* get fuel amount and sum */
    let total_mass: i32 = masses.iter().map(|&m| fuel_from_mass(m)).sum();
    println!("{}", total_mass);
    Ok(())
}
