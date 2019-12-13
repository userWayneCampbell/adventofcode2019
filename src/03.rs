use std::collections::HashMap;

pub fn min_intersections(str_in: String) -> i32 {
    let mut grid = HashMap::new();
    let mut intersections = Vec::new();

    for (n, line) in str_in.lines().enumerate() {
        let (mut x, mut y) = (0, 0);
        for (dir, num) in line.split(',').map(|a| a.split_at(1)) {
            for _ in 0..num.parse::<i32>().unwrap() {
                match dir {
                    "U" => y -= 1,
                    "D" => y += 1,
                    "L" => x -= 1,
                    "R" => x += 1,
                    _ => panic!("send help santa"),
                }

                /* create a hashmap of x,y that have been marked */
                if !grid.contains_key(&(x, y)) {
                    grid.insert((x, y), n);
                }
                /* if that x,y is already recorded, this is an intersection */
                else {
                    /* Only add intersection if not current line */
                    if grid.get(&(x,y)).unwrap() != &n {
                        intersections.push((x, y));
                    }
                }
            }
        }
    }
    /* get the Manhattan distance */
    let min = intersections
        .iter()
        .map(|(x, y)| i32::abs(*x) + i32::abs(*y))
        .min();

    min.unwrap()
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("../data/03.in");

    let min = min_intersections(contents.to_string());
    println!("part1: {}", min);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let contents = include_str!("../data/03_test.in");

        let min = min_intersections(contents.to_string());
        assert_eq!(min, 6);
    }

    #[test]
    fn test_two() {
        let contents = include_str!("../data/03_test1.in");

        let min = min_intersections(contents.to_string());
        assert_eq!(min, 159);
    }

    #[test]
    fn test_three() {
        let contents = include_str!("../data/03_test2.in");

        let min = min_intersections(contents.to_string());
        assert_eq!(min, 135);
    }
}
