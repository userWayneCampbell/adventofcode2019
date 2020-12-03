fn main() {
    let input = std::fs::read_to_string("data/02.in").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn p(input: &str) -> (Vec<&str>, Vec<usize>) {
    let words: Vec<_> = input.split_ascii_whitespace().collect();
    let lengths: Vec<_> = words[0]
        .split('-')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    (words, lengths)
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let (words, lengths) = p(line);

        let name = words[1].trim_end_matches(':');
        let passwords = words[2];

        let count = passwords.matches(name).count();
        if count >= lengths[0] && count <= lengths[1] {
            result += 1;
        }
    }

    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let (words, lengths) = p(line);

        let name = words[1].trim_end_matches(':').chars().next().unwrap();
        let passwords: Vec<_> = words[2].chars().collect();

        if (passwords[lengths[0] - 1] == name) != (passwords[lengths[1] - 1] == name) {
            result += 1;
        }
    }
    result
}
