impl Solution {
    
    fn rev_digits(x: i32) -> Vec<usize> {
        if x == 0 {
            return vec![0];
        }

        let mut ret = Vec::new();
        let mut eat_x = x;

        while eat_x > 0 {
            ret.push((eat_x % 10).try_into().unwrap());
            eat_x /= 10;
        }
        assert!(ret.len() > 0);

        ret
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false };
        let mut rd = Self::rev_digits(x);
        rd.iter().eq(rd.iter().rev())
    }
}
