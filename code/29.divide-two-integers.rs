impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let mut n_idend = if dividend > 0 { -dividend } else { dividend };
        let n_isor = if divisor > 0 { -divisor } else { divisor };

        let mut result = 0;

        while n_idend <= n_isor {
            
            let mut current_divisor = n_isor;
            let mut current_multiple = -1;

            while current_divisor >= (i32::MIN >> 1) && n_idend <= (current_divisor << 1) {
                current_divisor <<= 1;
                current_multiple <<= 1;
            }

            n_idend -= current_divisor;
            result += current_multiple;
        }

        if (dividend > 0) == (divisor > 0) {
             -result
        } else {
            result
        }
    }
}
