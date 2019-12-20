fn do_phase(str_in: String) -> String {
    let num_vec: Vec<u32> = str_in
        .chars()
        .map(|a| a.to_digit(10).unwrap())
        .collect();

    let mut str_out: String = "".to_string();

    for iteration in 1..=num_vec.len() {
        let mul_vec: Vec<i32> = generate_vec(iteration, num_vec.len());

        /* dot product */
        let result: Vec<i32> = num_vec
            .iter()
            .zip(mul_vec.iter())
            .map(|(x, y)| *x as i32 * y)
            .collect();

        let sum: i32 = result.iter().sum();
        str_out.push(sum.abs().to_string().chars().last().unwrap());
    }
    str_out
}

fn generate_vec(phase_num: usize, length: usize) -> Vec<i32> {
    let pattern = vec![0, 1, 0, -1];
    let mut return_vector = Vec::new();
    let mut modulus = 0;
    let mut len = 0;
    'outer: loop {
        for _ in 0..phase_num {
            return_vector.push(pattern[modulus % 4]);

            len += 1;
            if len > length {
                break 'outer;
            }
        }
        modulus += 1;
    }
    return_vector.remove(0);
    return_vector
}

fn main() -> std::io::Result<()> {
    let mut input = include_str!("../data/16.in").to_string();
    println!("{}", input);
    input.pop();
    for _ in 1..=100 {
        input = do_phase(input);
    }
    println!("{}", input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vec() {
        let result_1 = vec![1, 0, -1, 0, 1, 0, -1, 0];
        let test_1 = generate_vec(1, result_1.len());
        assert_eq!(result_1, test_1);

        let result_2 = vec![0, 1, 1, 0, 0, -1, -1, 0];
        let test_2 = generate_vec(2, result_2.len());
        assert_eq!(result_2, test_2);

        let result_3 = vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1];
        let test_3 = generate_vec(2, result_3.len());
        assert_eq!(result_3, test_3);

        let result_3 = vec![
            0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1,
            -1, -1, -1, -1, 0,
        ];
        let test_3 = generate_vec(8, result_3.len());
        assert_eq!(result_3, test_3);
    }

    #[test]
    fn example_one() {
        let contents = String::from("12345678");

        let result = do_phase(contents);
        assert_eq!(result, "48226158");

        let result = do_phase(result);
        assert_eq!(result, "34040438");

        let result = do_phase(result);
        assert_eq!(result, "03415518");

        let result = do_phase(result);
        assert_eq!(result, "01029498");
    }

    #[test]
    fn test_one() {
        let mut input = String::from("80871224585914546619083218645595");
        for _ in 1..=100 {
            input = do_phase(input);
        }
        assert_eq!("24176176", &input[..8]);
    }

    #[test]
    fn test_two() {
        let mut input = String::from("19617804207202209144916044189917");
        for _ in 1..=100 {
            input = do_phase(input);
        }
        assert_eq!("73745418", &input[..8]);
    }

    #[test]
    fn test_three() {
        let mut input = String::from("69317163492948606335995924319873");
        for _ in 1..=100 {
            input = do_phase(input);
        }
        assert_eq!("52432133", &input[..8]);
    }
}
