impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut n = n as i64;
        if n == 0 {
            return 1.0;
        }
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        let mut acc_n = 0;
        let mut acc_res = 1.0;

        while acc_n != n {
            let mut temp_n = 1;
            let mut temp_res = x;
            let mut l_temp_res = temp_res;

            while temp_n + acc_n <= n {
                l_temp_res = temp_res;
                temp_res *= temp_res;
                temp_n += temp_n;
            }

            temp_n /= 2;
            temp_res = l_temp_res;

            acc_n += temp_n;
            acc_res *= temp_res;
            assert!(acc_n <= n);
        }

        acc_res
    }
}
