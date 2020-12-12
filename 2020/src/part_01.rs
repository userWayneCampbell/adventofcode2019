pub fn one() -> (u32, u32) {
    let input = std::fs::read_to_string("data/01.in").unwrap();
    let memory: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // part 1
    let mut part1 = 0;
    for mem in memory.clone() {
        if let Some(num) = memory.iter().find(|&&a| a + mem == 2020) {
            part1 = num * num;
            break;
        }
    }

    // part 2
    for mem1 in memory.clone() {
        for mem2 in &memory {
            for mem3 in &memory {
                if mem1 + mem2 + mem3 == 2020 {
                    let part2 = mem1 * mem2 * mem3;
                    return (part1, part2);
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!((29929, 244300320), one());
    }
}
