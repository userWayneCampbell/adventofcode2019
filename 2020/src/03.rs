use std::convert::TryFrom;

fn main() {
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
    let second = calculate_trees(&tiles, 3, 1);
    println!("part1: {}", second);

    // part 2
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut product = 1;
    for (right, down) in slopes {
        let a = calculate_trees(&tiles, right, down);
        product *= a;
    }
    println!("part2: {}", product);
}

fn calculate_trees(tiles: &[Vec<Tile>], right: usize, down: usize) -> usize {
    let ys: Vec<usize> = (1..tiles.len())
        .map(|a| a * down)
        .filter(|&a| a <= tiles.len())
        .collect();
    let xs: Vec<usize> = (1..=ys.len())
        .map(|a| (a * right) % tiles[0].len())
        .collect();

    let mut num_ouch: usize = 0;
    for (x, y) in ys.iter().zip(&xs) {
        let tile = &tiles[*x as usize][*y as usize];
        if matches!(tile, Tile::Tree) {
            num_ouch += 1;
        }
    }
    num_ouch
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
