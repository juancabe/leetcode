impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let trimmed = s.trim();
        let mut chars = trimmed.chars().peekable();
        let sign = match chars.peek() {
            None => return 0,
            Some(s) => match s {
                '+' => { chars.next(); 1 },
                '-' => { chars.next(); -1 },
                d => {
                    if d.is_digit(10) { 1 }
                    else { return 0 }
                }
            }
        };

        while let Some(z) = chars.peek() && *z == '0' {
            chars.next();
        }

        match chars.peek() {
            None => return 0,
            Some(nd) => {
                if !nd.is_digit(10) {
                    return 0
                }
            }
        }

        let mut final_chars = Vec::new();

        while let Some(digit) = chars.next() && digit.is_digit(10) {
            final_chars.push(digit);
        }

        if final_chars.len() > 10 {
            if sign == 1 {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        let parsed: u128 = (final_chars.into_iter().collect::<String>()).parse().unwrap();

        match TryInto::<i32>::try_into(parsed) {
            Ok(p) => sign * p,
            Err(_) => if sign == 1 { i32::MAX } else { i32::MIN }
        }
        
    }
}
