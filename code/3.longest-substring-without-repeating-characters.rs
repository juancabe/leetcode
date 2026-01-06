impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        
        if s.is_empty() { return 0 };
        let s = s.as_bytes();

        let mut max_long = 1;
        let mut seen_at = [None; 255];
        let mut s_i = -1;

        for i in 0..s.len() {

            let new_b = s[i] as usize;

            if let Some(last_seen) = seen_at[new_b] && last_seen as i32 > s_i  {
                s_i = last_seen as i32;
            } 

            seen_at[new_b] = Some(i);
            max_long = max_long.max((i as i32 - s_i).try_into().unwrap());
        }

        max_long as i32
    }
}
