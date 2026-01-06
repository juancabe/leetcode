impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        if !p.is_empty() && p.iter().all(|c| *c == '*') { return true };

        fn keep_matching(s: &[char], p: &[char]) -> bool {
            if s.is_empty() && p.iter().all(|c| *c == '*') {
                return true;
            }
            if s.is_empty() || p.is_empty() {
                return false;
            }

            match p.first() {
                Some('?') => (),
                Some('*') => {
                    return keep_matching(&s, &p[1..]) 
                        || keep_matching(&s[1..], &p)
                        || keep_matching(&s[1..], &p[1..]);
                },
                Some(c) => {
                    if s[0] != *c {
                        return false; 
                    }
                },
                None => unreachable!(),
            }

            keep_matching(&s[1..], &p[1..])
        }

        keep_matching(&s, &p)
    }
}
