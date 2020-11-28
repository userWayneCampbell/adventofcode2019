use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

fn count_orbits(orbits: &HashMap<&str, &str>, key: &str, orbit_count: &mut usize) {
    if orbits.contains_key(key) {
        if let Some(result) = orbits.get(key) {
            print!("<-{}", result);
            *orbit_count += 1;
            count_orbits(orbits, result, orbit_count);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = match File::open("data/06.in") {
        Err(err) => panic!("Couldn't read file : {}", err),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    /* create hashmap of planets from right to left */
    let mut orbits = HashMap::new();
    for line in contents.lines() {
        let v: Vec<&str> = line.split(')').collect();
        orbits.insert(v[1], v[0]);
    }
    println!("{:?}", orbits);

    let mut orbit_count: usize = 0;

    /* increment through starting planet finding longest possible path */
    for (right, left) in &orbits {
        /* increment for orbit in hashmap */
        orbit_count += 1;
        print!("{}<-{}", (*right).to_string(), (*left).to_string());
        /* recursive function to increment orbit_count for all found planets */
        count_orbits(&orbits, left, &mut orbit_count);
        println!();
    }

    println!("{}", orbit_count);
    Ok(())
}
