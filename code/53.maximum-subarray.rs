#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 52.741985%

//  Given an integer array nums, find the subarray with the largest sum, and return its sum.
//
//
//  Example 1:
//
//  Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//  Output: 6
//  Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//
//
//  Example 2:
//
//  Input: nums = [1]
//  Output: 1
//  Explanation: The subarray [1] has the largest sum 1.
//
//
//  Example 3:
//
//  Input: nums = [5,4,-1,7,8]
//  Output: 23
//  Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//
//
//
//  Constraints:
//
//
//  	1 <= nums.length <= 10⁵
//  	-10⁴ <= nums[i] <= 10⁴
//
//
//
//  Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut acc = 0;
        let mut max: i32 = nums[0];
        for n in nums {
            acc = n.max(acc + n);
            max = max.max(acc);
        }
        max
    }
}
//  // @submit end

fn main() {
    let res = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    println!("res: {res}");
}
