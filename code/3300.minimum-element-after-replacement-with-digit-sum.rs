//  Category: algorithms
//  Level: Easy
//  Percent: 84.43345%

//  You are given an integer array nums.
//
//  You replace each element in nums with the sum of its digits.
//
//  Return the minimum element in nums after all replacements.
//
//
//  Example 1:
//
//
//  Input: nums = [10,12,13,14]
//
//  Output: 1
//
//  Explanation:
//
//  nums becomes [1, 3, 4, 5] after all replacements, with minimum element 1.
//
//
//  Example 2:
//
//
//  Input: nums = [1,2,3,4]
//
//  Output: 1
//
//  Explanation:
//
//  nums becomes [1, 2, 3, 4] after all replacements, with minimum element 1.
//
//
//  Example 3:
//
//
//  Input: nums = [999,19,199]
//
//  Output: 10
//
//  Explanation:
//
//  nums becomes [27, 10, 19] after all replacements, with minimum element 10.
//
//
//
//  Constraints:
//
//
//  	1 <= nums.length <= 100
//  	1 <= nums[i] <= 10⁴
//

//  // @submit start
impl Solution {
    #[inline]
    fn sum_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }

    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for n in nums {
            min = min.min(Self::sum_digits(n));
        }
        min
    }
}
//  // @submit end
