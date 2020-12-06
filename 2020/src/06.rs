fn main() {
    let input = std::fs::read_to_string("data/06.in").unwrap();

    let groups: Vec<(usize, Vec<char>)> = input
        .split("\n\n")
        .map(|entry| {
            let group: Vec<char> = entry
                .lines()
                .flat_map(str::chars)
                .collect();
            (entry.lines().count(), group)
        })
        .collect();

    // part1
    let mut part1_groups = groups.clone();
    let part1: usize = part1_groups
        .iter_mut()
        .map(|(_, group)| {
            group.sort_unstable();
            group.dedup();
            group.len()
        })
        .sum();
    assert_eq!(6504, part1);
    println!("part1: {:?}", part1);

    let mut part2_groups = groups;
    let part2: usize = part2_groups
        .iter_mut()
        .map(|(num_people, group)| {
            let original = group.clone();
            group.sort_unstable();
            group.dedup();
            group
                .iter()
                .map(|g| (original.iter().filter(|a| *a == g).count() == *num_people) as usize)
                .sum::<usize>()
        })
        .sum();
    println!("part2: {}", part2);
    assert_eq!(3351, part2);
}
