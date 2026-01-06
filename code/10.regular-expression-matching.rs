use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        // Memoization table to store results of (s_index, p_index)
        let mut memo = HashMap::new();
        
        return Self::dfs(0, 0, &s_chars, &p_chars, &mut memo);
    }

    fn dfs(i: usize, j: usize, s: &Vec<char>, p: &Vec<char>, memo: &mut HashMap<(usize, usize), bool>) -> bool {
        // 1. Check Memoization
        if let Some(&res) = memo.get(&(i, j)) {
            return res;
        }

        // 2. Base Case: Reached end of Pattern
        if j == p.len() {
            return i == s.len();
        }

        // 3. Match: Check if current char matches (or is '.')
        let first_match = i < s.len() && (p[j] == s[i] || p[j] == '.');

        // 4. Star Logic: Look ahead
        let ans = if j + 1 < p.len() && p[j+1] == '*' {
            // Option A: Skip star (match 0 times)
            // Option B: Use star (match 1 time) -> strictly requires first_match is true
            Self::dfs(i, j + 2, s, p, memo) || 
            (first_match && Self::dfs(i + 1, j, s, p, memo))
        } else {
            // Normal Logic: strict match required
            first_match && Self::dfs(i + 1, j + 1, s, p, memo)
        };

        // 5. Save and Return
        memo.insert((i, j), ans);
        ans
    }
}
