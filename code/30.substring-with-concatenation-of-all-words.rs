// fn backtrack(
//         s: &str, mut curr: String,
//         r_len: usize, remaining: u64,
//         words: &[String],
//         concatenated: &mut Vec<usize>) -> String {
// 
//     if r_len == 0 {
//         let mut s_c = s;
//         let mut removed = 0;
//         while let Some(pos) = s_c.find(&curr) {
//             concatenated.push(pos + removed);
//             removed += pos + 1;
//             s_c = &s_c[pos + 1..];
//         }
//         return curr;
//     }
// 
//     if !s.contains(&curr) {
//         return curr;
//     }
// 
//     let curr_len = curr.len();
// 
//     for disp in 0..words.len() {
//         let remains = (1 & remaining >> disp) == 1;
//         if remains {
//             curr.push_str(&words[disp]);
//             let new_remaining = remaining ^ (1 << disp);
//             curr = backtrack(s, curr, r_len - 1, new_remaining, words, concatenated);
//             curr.truncate(curr_len);
//         }
//     }
// 
//     curr
// }

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s_bytes = s.as_bytes();
        let word_len = words[0].len();
        let words_count = words.len();
        let total_len = word_len * words_count;
        let s_len = s.len();

        if s_len < total_len {
            return vec![];
        }

        let mut target_map: HashMap<&[u8], i32> = HashMap::new();
        for word in words.iter() {
            *target_map.entry(word.as_bytes()).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        let mut current_map: HashMap<&[u8], i32> = HashMap::new();

        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut count = 0;

            current_map.clear();
            while right + word_len <= s_len {
                let w = &s_bytes[right..right + word_len];
                right += word_len;

                if let Some(&target_count) = target_map.get(w) {
                    *current_map.entry(w).or_insert(0) += 1;
                    count += 1;

                    while current_map[w] > target_count {
                        let left_w = &s_bytes[left..left + word_len];
                        *current_map.get_mut(left_w).unwrap() -= 1;
                        count -= 1;
                        left += word_len;
                    }

                    if count == words_count {
                        result.push(left as i32);
                    }

                } else {
                    current_map.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        result
    }
}
