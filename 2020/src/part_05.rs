pub fn five() -> (usize, usize) {
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
    let part1 = max;

    for x in *min..=*max {
        if !seats.contains(&x) {
            return (*part1, x)
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five() {
        assert_eq!((989, 548), five());
    }
}
