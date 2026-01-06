impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue = Vec::new();

        fn is_closing(c: char) -> bool {
            match c {
                '}' | ')' | ']' => true,
                _ => false
            }
        }

        fn opens(c: char) -> char {
            match c {
                '}' => '{',
                ')' => '(',
                ']' => '[',
                _ => panic!()
            }
        }

        for p in s.chars() {
            if is_closing(p) {
                if let Some(o) = queue.pop() && o == opens(p) {
                    continue;
                } else {
                    return false;
                }
            } else {
                queue.push(p)
            }
        }

        queue.is_empty()
        
    }
}
