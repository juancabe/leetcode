impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let mut max: [Option<i64>; 4] = [None; 4];

        for (i, b_val) in b.iter().enumerate() {
            for e in (0..=i.min(3)).rev() {
                let prev = if e == 0 { 0 } else { max[e - 1].unwrap() };
                max[e] = match max[e] {
                    Some(max_e) => {
                        Some(max_e.max(prev + (a[e] as i64 * *b_val as i64)))
                    },
                    None => {
                        Some(prev + (a[i] as i64 * *b_val as i64))
                    }
                }
            }
        }

        max.last().unwrap().unwrap()
    }
}
