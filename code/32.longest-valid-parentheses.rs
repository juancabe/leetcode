impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        
        // Pasada 1: Izquierda -> Derecha
        let mut left = 0;
        let mut right = 0;
        for &c in chars.iter() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                max_len = max_len.max(left + right);
            } else if right > left {
                // Inválido: hay más cerrados que abiertos. Reseteamos.
                left = 0;
                right = 0;
            }
        }

        // Pasada 2: Derecha -> Izquierda
        left = 0;
        right = 0;
        for &c in chars.iter().rev() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }

            if left == right {
                max_len = max_len.max(left + right);
            } else if left > right {
                // Inválido: hay más abiertos que cerrados. Reseteamos.
                left = 0;
                right = 0;
            }
        }

        max_len
    }
}
