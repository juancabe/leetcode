impl Solution {
    
    fn d_letters(d: char) -> &'static [char] {
        match d {
            '2' => &['a', 'b', 'c'],
            '3' => &['d', 'e', 'f'],
            '4' => &['g', 'h', 'i'],
            '5' => &['j', 'k', 'l'],
            '6' => &['m', 'n', 'o'],
            '7' => &['p', 'q', 'r', 's'],
            '8' => &['t', 'u', 'v'],
            '9' => &['w', 'x', 'y', 'z'],
            _ => panic!()
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        let mut digits = digits.chars().map(|c| Self::d_letters(c));

        for letter in digits.next().unwrap() {
            ret.push(letter.to_string());
        }

        while let Some(digit) = digits.next() {
            let mut new_vec = Vec::new();

            for letter in digit {
                for vec in &ret {
                    let mut inner = vec.clone();
                    inner.push(*letter);
                    new_vec.push(inner);
                }
            }

            ret = new_vec;
        }

        ret
    }
}
