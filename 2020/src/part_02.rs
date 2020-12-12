pub fn two() -> (usize, usize) {
    let input = std::fs::read_to_string("data/02.in").unwrap();
    let part1 = part1(&input);

    let part2 = part2(&input);
    (part1, part2)
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
    input
        .lines()
        .map(|line| {
            let (words, lengths) = p(line);

            let name = words[1].trim_end_matches(':');
            let passwords = words[2];

            let count = passwords.matches(name).count();
            if count >= lengths[0] && count <= lengths[1] {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (words, lengths) = p(line);

            let name = words[1].trim_end_matches(':').chars().next().unwrap();
            let passwords: Vec<_> = words[2].chars().collect();

            if (passwords[lengths[0] - 1] == name) == (passwords[lengths[1] - 1] == name) {
                0
            } else {
                1
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        assert_eq!((586, 352), two());
    }
}
