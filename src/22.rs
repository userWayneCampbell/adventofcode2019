struct Deck {
    v: Vec<usize>,
}

impl Deck {
    fn cut(&mut self, num: i32) {
        if num.is_positive() {
            let mut beginning = self.v[0..num.abs() as usize].to_vec();
            let mut end = self.v[num.abs() as usize..].to_vec();
            end.append(&mut beginning);
            self.v = end;
            //end
        }
        else {
            println!("negative {}", num);
            let mut beginning = self.v[(self.v.len() - num.abs() as usize)..].to_vec();
            let mut end = self.v[0..(self.v.len() - num.abs() as usize)].to_vec();
            beginning.append(&mut end);
            println!("negative {:?} ", beginning);
            self.v = beginning;
            //beginning
        }
    }
    fn increment(&mut self, num: usize) {
        let mut position = 0;
        let mut out_vector = vec![0; self.v.len()];
        for n in 0..self.v.len() {
            out_vector[position] = self.v[n];
            position = (position + num) % self.v.len();
        }
        self.v = out_vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_N_cards() {
        // positive
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.cut(3);

        let v_exp: Vec<usize> = vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        assert_eq!(v_exp, d.v);

        // negative
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.cut(-4);

        let v_exp: Vec<usize> = vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_increment() {
        let v_inp: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.increment(3);

        let v_exp: Vec<usize> = vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_new_stack() {
        let v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.v.reverse();

        let v_exp = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_one() {
        let v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.increment(7);
        d.v.reverse();
        d.v.reverse();

        let v_exp = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_two() {
        let v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.cut(6);
        d.increment(7);
        d.v.reverse();

        let v_exp = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_three() {
        let v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.increment(7);
        d.increment(9);
        d.cut(-2);

        let v_exp = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        assert_eq!(v_exp, d.v);
    }

    #[test]
    fn test_four() {
        let v_inp = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut d = Deck { v: v_inp };

        d.v.reverse();
        d.cut(-2);
        d.increment(7);
        d.cut(8);
        d.cut(-4);
        d.increment(7);
        d.cut(3);
        d.increment(9);
        d.increment(3);
        d.cut(-1);

        let v_exp = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        assert_eq!(v_exp, d.v);
    }
}
