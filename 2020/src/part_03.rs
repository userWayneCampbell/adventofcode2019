use std::convert::TryFrom;

pub fn three() -> (usize, usize) {
    let input = std::fs::read_to_string("data/03.1.in").unwrap();

    let mut tiles = vec![];
    for line in input.lines() {
        let mut tile_line = vec![];
        for c in line.chars() {
            tile_line.push(Tile::try_from(c).unwrap());
        }
        tiles.push(tile_line);
    }

    // part 1
    let part1 = calculate_trees(&tiles, 3, 1);

    // part 2
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2: usize = slopes
        .iter()
        .map(|(right, down)| calculate_trees(&tiles, *right, *down))
        .product();

    (part1, part2)
}

fn calculate_trees(tiles: &[Vec<Tile>], right: usize, down: usize) -> usize {
    let ys: Vec<usize> = (1..tiles.len())
        .map(|a| a * down)
        .filter(|&a| a <= tiles.len())
        .collect();
    let xs: Vec<usize> = (1..=ys.len())
        .map(|a| (a * right) % tiles[0].len())
        .collect();

    ys.iter()
        .zip(&xs)
        .map(|(x, y)| {
            let tile = &tiles[*x as usize][*y as usize];
            matches!(tile, Tile::Tree) as usize
        })
        .sum()
}

#[derive(Debug)]
pub enum Tile {
    Square,
    Tree,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Square),
            '#' => Ok(Self::Tree),
            _ => Err("oh no"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        assert_eq!((207, 2_655_892_800), three());
    }
}
