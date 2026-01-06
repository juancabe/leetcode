impl Solution {
    

    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {

        let mut chars_cost: [i64; 255] = [0i64; 255];

        for (i, c) in s.chars().enumerate() {
            chars_cost[c as usize] += cost[i] as i64;
        }

        let char_keep = chars_cost.iter().enumerate()
            .max_by_key(|(_, c)| *c)
            .map(|(ascii, _)| ascii)
            .unwrap();

        chars_cost.iter().enumerate()
            .filter(|(i, _)| *i != char_keep)
            .map(|(_, c)|c )
            .sum()
    }
}
