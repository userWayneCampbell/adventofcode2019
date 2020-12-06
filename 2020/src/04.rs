fn main() {
    let input = std::fs::read_to_string("data/04.in").unwrap();

    let mut num = 0;
    for entry in input.split("\n\n") {
        if entry.contains("byr")
            && entry.contains("iyr")
            && entry.contains("eyr")
            && entry.contains("hgt")
            && entry.contains("hcl")
            && entry.contains("ecl")
            && entry.contains("pid")
        {
            num += 1;
        }
    }
    println!("part1: {}", num);

    let mut total_correct = 0;
    for line in input.split("\n\n") {
        let mut f = [false; 7];
        for entry in line.split_whitespace() {
            if entry.is_empty() {
                continue;
            }
            match &entry[..3] {
                "byr" => f[0] = (1920..=2002).contains(&entry[4..].parse::<usize>().unwrap()),
                "iyr" => f[1] = (2010..=2020).contains(&entry[4..].parse::<usize>().unwrap()),
                "eyr" => f[2] = (2020..=2030).contains(&entry[4..].parse::<usize>().unwrap()),
                "hgt" => {
                    f[3] = match &entry[entry.len() - 2..] {
                        "in" => {
                            (59..=76).contains(&entry[4..entry.len() - 2].parse::<usize>().unwrap())
                        }
                        "cm" => (150..=193)
                            .contains(&entry[4..entry.len() - 2].parse::<usize>().unwrap()),
                        _ => false,
                    }
                }
                "hcl" => {
                    f[4] = entry[5..].len() == 6
                        && &entry[4..5] == "#"
                        && entry[5..].chars().all(|x| {
                            (b'0'..=b'9').contains(&(x as u8)) || (b'a'..=b'f').contains(&(x as u8))
                        })
                }
                "ecl" => {
                    f[5] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&entry[4..])
                }
                "pid" => {
                    f[6] = entry[4..].len() == 9
                        && entry[4..].bytes().all(|x| (b'0'..=b'9').contains(&x))
                }
                _ => (),
            }
        }
        if f[0] && f[1] && f[2] && f[3] && f[4] && f[5] && f[6] {
            total_correct += 1;
        }
    }
    println!("part2: {}", total_correct);
}
