impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        if s.is_empty() {
            return "".to_string()
        }

        fn is_p(s: &str) -> &str {
            let sb = s.as_bytes();

            let o = if s.len() % 2 == 0 { 1 } else { 0 };

            let mut i;
            let mut j;

            if s.len() % 2 == 0 {
                i = (sb.len() / 2) as i32 - 1;
                j = sb.len() / 2;
            } else {
                let center = sb.len() / 2;
                i = (center as i32) - 1;
                j = center + 1;
            }

            while j < sb.len() {
                if sb[i as usize] != sb[j] {
                    return &s[(i + 1) as usize..j];
                }
                i -= 1;
                j += 1;
            }

            s
        }

        let mid = s.len() / 2;
        let mut max_str = &s[0..1];
        for i in 0..s.len() - 1 {
            let s1 = &s[..s.len() - i];
            let s2 = &s[i..];
            let s1 = is_p(s1);
            let s2 = is_p(s2);

            if s1.len() > max_str.len() {
                max_str = s1;
            }

            if s2.len() > max_str.len() {
                max_str = s2;
            }
        }

        max_str.to_string()
    }
}
