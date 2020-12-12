pub fn nine() -> (usize, usize) {
    let input = std::fs::read_to_string("data/09.in").unwrap();
    let input: Vec<usize> = input.lines().map(|a| a.parse().unwrap()).collect();

    let mut part1 = 0;
    for window in input.windows(26) {
        let preamble = &window[..25];
        let num = &window[25];

        let mut found = false;
        for x in preamble {
            for y in preamble {
                if x != y && *x + *y == *num {
                    found = true;
                }
            }
        }
        if !found {
            part1 = *num;
            break;
        }
    }

    let mut part2 = 0;
    for i in 2..input.len() {
        for window in input.windows(i) {
            if window.iter().sum::<usize>() == part1 {
                let mut window = window.to_owned();
                window.sort_unstable();
                part2 = window.first().unwrap() + window.last().unwrap();
            }
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nine() {
        assert_eq!((466456641, 55732936), nine());
    }
}
