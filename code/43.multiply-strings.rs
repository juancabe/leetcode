impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let (num1, num2) = if num1 >= num2 { (num1, num2) } else { (num2, num1) };

        let num1i = num1.chars().rev();
        let num2i = num2.chars().rev();

        if num2 == "0" { return "0".to_string() };

        let mut ret = vec![0u32; num1.len() + num2.len()];

        for (i, d2) in num2i.enumerate() {

            let mut carry = 0;
            let mut last_j = 0;

            for (j, d1) in num1i.clone().enumerate() {
                let d1 = d1.to_digit(10).unwrap();
                let d2 = d2.to_digit(10).unwrap();
                let mut res = d1 * d2 + carry;
                carry = res / 10;
                res %= 10;

                let ret_idx = i + j;

                ret[ret_idx] += res;
                carry += ret[ret_idx] / 10;
                ret[ret_idx] %= 10;

                last_j = j;
            }
            last_j += 1;
            ret[i + last_j] += carry;
        }

        let skip = if *ret.last().unwrap() != 0 { 0 } else { 1 };

        String::from_iter(ret.iter().rev().skip(skip).map(|d| d.to_string()))
    }
}
