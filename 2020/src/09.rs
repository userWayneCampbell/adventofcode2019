fn main() {
    let input = std::fs::read_to_string("data/09.in").unwrap();
    let input: Vec<usize> = input.lines().map(|a| a.parse().unwrap()).collect();

    let mut part1 = 0;
    for window in input.windows(26) {
        let preamble = &window[..25];
        let num = &window[25];

        let mut found = false;
        for x in preamble {
            for y in preamble {
                if x != y {
                    if *x + *y == *num {
                        found = true;
                    }
                }
            }
        }
        if found == false {
            part1 = *num;
            break;
        }
    }
    println!("part1: {}", part1);

    let mut part2 = 0;
    for i in 2..input.len() {
        for window in input.windows(i) {
            if window.iter().sum::<usize>() == part1 {
                let mut window = window.to_owned();
                window.sort();
                part2 = window.first().unwrap() + window.last().unwrap();
            }
        }
    }
    println!("part2: {}", part2);
}
