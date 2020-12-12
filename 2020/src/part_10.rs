#[must_use] pub fn ten() -> (usize, usize) {
    let input = std::fs::read_to_string("data/10.in").unwrap();
    let mut input: Vec<usize> = input.lines().map(|a| a.parse().unwrap()).collect();

    input.sort_unstable();
    let last = input.last().unwrap() + 3;
    input.push(last);

    let mut one_jolt = 0;
    let mut three_jolt = 0;
    let mut last_jolt = 0;

    for i in &input {
        let diff = i - last_jolt;

        match diff {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => unreachable!(),
        }
        last_jolt = *i;
    }

    let part1 = (three_jolt) * one_jolt;

    input.push(0);
    input.sort_unstable();

    let mut results = std::collections::HashMap::new();
    let part2 = maybe_good_possible_results(&input, 0, &mut results);

    (part1, part2)
}

fn maybe_good_possible_results(
    adapters: &[usize],
    current: usize,
    results: &mut std::collections::HashMap<usize, usize>,
) -> usize {
    if current == adapters.len() - 1 {
        return 1;
    }

    if let Some(value) = results.get(&current) {
        return *value;
    }

    let possibilities: usize = ((current + 1)..adapters.len())
        .take_while(|index| adapters[*index] <= adapters[current] + 3)
        .map(|index| maybe_good_possible_results(adapters, index, results))
        .sum();

    results.insert(current, possibilities);

    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ten() {
        assert_eq!((2590, 226_775_649_501_184), ten());
    }
}
