pub fn cut(num: f64, v: &Vec<usize>) -> Vec<usize> {
    if num.is_sign_positive() {
        let mut beginning = v[0..num.abs().trunc() as usize].to_vec();
        let mut end = v[num.abs().trunc() as usize..].to_vec();
        end.append( &mut beginning);
        end
    }
    else {
        println!("negative {}", num);
        let mut beginning = v[(v.len() - num.abs().trunc() as usize)..].to_vec();
        let mut end = v[0..(v.len() - num.abs().trunc() as usize)].to_vec();
        beginning.append( &mut end);
        println!("negative {:?} ", beginning);
        beginning
    }
}

pub fn increment(num: usize, v: &Vec<usize>) -> Vec<usize> {
    let mut position = 0;
    let mut out_vector = vec![0; v.len()];
    for n in 0..v.len() {
        out_vector[position] = v[n];
        position = (position + num) % v.len();
    }
    out_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_N_cards() {
        // positive
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let v_exp: Vec<usize> = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let out: Vec<usize> = cut(3.0, &v_inp);
        assert_eq!(v_exp, out);

        // negative
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let v_exp: Vec<usize> = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        let out: Vec<usize> = cut(-4.0, &v_inp);
        assert_eq!(v_exp, out);
    }

    #[test]
    fn test_increment() {
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let out: Vec<usize> = increment(3, &v_inp);
        let v_exp: Vec<usize> = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        assert_eq!(v_exp, out);
    }

    #[test]
    fn test_new_stack() {
        let mut v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        v_inp.reverse();
        let v_exp = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(v_inp, v_exp);
    }
}
