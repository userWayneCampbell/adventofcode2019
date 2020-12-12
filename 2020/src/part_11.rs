const FILLED: char = '#';
const EMPTY: char = 'L';
const FLOOR: char = '.';

pub enum Part {
    One,
    Two,
}

impl Part {
    /// return the amount of filled neighbors required before evaluating
    fn filled_neighbors(&self) -> (usize, usize) {
        match self {
            Self::One => (4, 0),
            Self::Two => (5, 0),
        }
    }

    /// step through until the current seats(last evaluated) == the new seats, then do summation of the '#'s
    pub fn steps(&self, seats: Vec<Vec<char>>) -> usize {
        let mut current_seats = seats;
        loop {
            let new_seats = self.step(&current_seats);
            if new_seats == current_seats {
                break;
            }
            current_seats = new_seats;
        }
        // sum of '#' in 2d vec
        current_seats
            .iter()
            .map(|x| x.iter().filter(|a| *a == &FILLED).count())
            .sum()
    }

    fn step(&self, seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_seats = seats.clone();

        let row_max = seats.len();
        let col_max = seats[0].len();

        // loop through every option, skipping every '.' and checking each adjacent neighbors if applicable.
        //
        // Then, fill and remove seats depending on the different part of the problem
        for row in 0..row_max {
            for col in 0..col_max {
                if seats[row][col] == FLOOR {
                    continue;
                }

                let xy = [
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ];

                match self {
                    Part::One => {
                        let mut filled_neighbors: usize = 0;
                        for (x, y) in &xy {
                            let new_x = (row as i64) + x;
                            let new_y = (col as i64) + y;

                            // find filled neighbors
                            if new_x >= 0
                                && new_x < row_max as i64
                                && new_y >= 0
                                && new_y < col_max as i64
                            {
                                let neighbor_row = new_x as usize;
                                let neighbor_col = new_y as usize;

                                filled_neighbors += match seats[neighbor_row][neighbor_col] {
                                    'L' | '.' => 0,
                                    '#' => 1,
                                    _ => unreachable!(),
                                }
                            }
                        }

                        // act upon filled neighbor count
                        let count_neighbors = self.filled_neighbors();
                        if seats[row][col] == FILLED {
                            // If a seat is occupied (#) and four or more seats adjacent to it are
                            // also occupied, the seat becomes empty.
                            if filled_neighbors >= count_neighbors.0 {
                                new_seats[row][col] = EMPTY;
                            }
                        // If a seat is empty (L) and there are no occupied seats adjacent to it,
                        // the seat becomes occupied.
                        } else if seats[row][col] == EMPTY && filled_neighbors == count_neighbors.1
                        {
                            new_seats[row][col] = FILLED
                        }
                    }
                    Part::Two => {
                        let mut filled_neighbors: usize = 0;
                        for (x, y) in &xy {
                            let mut new_x = row as i64;
                            let mut new_y = col as i64;
                            loop {
                                // starting at the base offset, just increase it each time by the
                                // offset thus 'moving' in one direction until hitting the last x
                                // and last y in either directions
                                new_x += x;
                                new_y += y;

                                if new_x >= 0
                                    && new_x < row_max as i64
                                    && new_y >= 0
                                    && new_y < col_max as i64
                                {
                                    let neighbor_row = new_x as usize;
                                    let neighbor_col = new_y as usize;

                                    // look for empty or neighbor, and breaking once found to have
                                    // some sort of 'line of sight' involved.
                                    //
                                    // it's 2020, they really should be farther away from each
                                    // other honestly maybe you dont wanna get a seat closest to
                                    // yourself? really? mabye? eh?
                                    match seats[neighbor_row][neighbor_col] {
                                        EMPTY => {
                                            // found first empty
                                            break;
                                        }
                                        FILLED => {
                                            // found first neighbor
                                            filled_neighbors += 1;
                                            break;
                                        }
                                        FLOOR => {
                                            continue;
                                        }
                                        _ => unreachable!(),
                                    }
                                } else {
                                    break;
                                }
                            }
                        }

                        if seats[row][col] == FILLED {
                            if filled_neighbors >= 5 {
                                new_seats[row][col] = EMPTY;
                            }
                        } else if seats[row][col] == EMPTY && filled_neighbors == 0 {
                            new_seats[row][col] = FILLED
                        }
                    }
                }
            }
        }
        new_seats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eleven() {
        let input = std::fs::read_to_string("data/11.in").unwrap();
        let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let part1 = Part::One.steps(seats.clone());
        assert_eq!(2211, part1);

        let part2 = Part::Two.steps(seats);
        assert_eq!(1995, part2);
    }
}
