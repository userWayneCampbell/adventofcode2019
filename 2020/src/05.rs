fn main() {
    let input = std::fs::read_to_string("data/05.in").unwrap();

    let seats: Vec<usize> = input
        .lines()
        .map(|line| {
            let s = line.replace("B", "1");
            let s = s.replace("F", "0");

            let s = s.replace("R", "1");
            let s = s.replace("L", "0");

            let column = usize::from_str_radix(&s[..7], 2).unwrap() << 3;
            let row = usize::from_str_radix(&s[7..10], 2).unwrap();
            column + row
        })
        .collect();

    let min = seats.iter().min().unwrap();
    let max = seats.iter().max().unwrap();
    println!("part1 max: {}", max);
    println!("min: {}", min);

    for x in *min..=*max {
        if !seats.contains(&x) {
            println!("part2: seat: {}", x);
        }
    }
}
