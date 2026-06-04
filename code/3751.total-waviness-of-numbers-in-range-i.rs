//  Category: algorithms
//  Level: Medium
//  Percent: 79.960976%

//  You are given two integers num1 and num2 representing an inclusive range [num1, num2].
//
//  The waviness of a number is defined as the total count of its peaks and valleys:
//
//
//  	A digit is a peak if it is strictly greater than both of its immediate neighbors.
//  	A digit is a valley if it is strictly less than both of its immediate neighbors.
//  	The first and last digits of a number cannot be peaks or valleys.
//  	Any number with fewer than 3 digits has a waviness of 0.
//
//  Return the total sum of waviness for all numbers in the range [num1, num2].
//
//  Example 1:
//
//
//  Input: num1 = 120, num2 = 130
//
//  Output: 3
//
//  Explanation:
//  In the range [120, 130]:
//
//
//  	120: middle digit 2 is a peak, waviness = 1.
//  	121: middle digit 2 is a peak, waviness = 1.
//  	130: middle digit 3 is a peak, waviness = 1.
//  	All other numbers in the range have a waviness of 0.
//
//
//  Thus, total waviness is 1 + 1 + 1 = 3.
//
//
//  Example 2:
//
//
//  Input: num1 = 198, num2 = 202
//
//  Output: 3
//
//  Explanation:
//  In the range [198, 202]:
//
//
//  	198: middle digit 9 is a peak, waviness = 1.
//  	201: middle digit 0 is a valley, waviness = 1.
//  	202: middle digit 0 is a valley, waviness = 1.
//  	All other numbers in the range have a waviness of 0.
//
//
//  Thus, total waviness is 1 + 1 + 1 = 3.
//
//
//  Example 3:
//
//
//  Input: num1 = 4848, num2 = 4848
//
//  Output: 2
//
//  Explanation:
//
//  Number 4848: the second digit 8 is a peak, and the third digit 4 is a valley, giving a waviness of 2.
//
//
//
//  Constraints:
//
//
//  	1 <= num1 <= num2 <= 10⁵
//

//  // @submit start
impl Solution {
    fn waviness(mut n: i32) -> i32 {
        const POW_10: [i32; 7] = [1, 10, 100, 1000, 10000, 100000, 1000000];

        let mut waviness = 0;

        let digit_count = (n.ilog10() + 1) as i32;
        let get_digit = |digit_index: i32| n / POW_10[(digit_count - digit_index) as usize] % 10i32;

        let mut digit_index = digit_count - 1;
        while digit_index > 1 {
            let current_digit = get_digit(digit_index);
            let prev_digit = get_digit(digit_index - 1);
            let next_digit = get_digit(digit_index + 1);

            if current_digit > prev_digit && current_digit > next_digit
                || current_digit < prev_digit && current_digit < next_digit
            {
                waviness += 1;
            }

            digit_index -= 1;
        }

        waviness
    }

    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut wav = 0;
        for i in num1..=num2 {
            wav += Self::waviness(i);
        }
        wav
    }
}
//  // @submit end
