//  Category: algorithms
//  Level: Easy
//  Percent: 87.76434%

//  You are given a 0-indexed integer array nums of size n.
//
//  Define two arrays leftSum and rightSum where:
//
//
//  	leftSum[i] is the sum of elements to the left of the index i in the array nums. If there is no such element, leftSum[i] = 0.
//  	rightSum[i] is the sum of elements to the right of the index i in the array nums. If there is no such element, rightSum[i] = 0.
//
//
//  Return an integer array answer of size n where answer[i] = |leftSum[i] - rightSum[i]|.
//
//
//  Example 1:
//
//  Input: nums = [10,4,8,3]
//  Output: [15,1,11,22]
//  Explanation: The array leftSum is [0,10,14,22] and the array rightSum is [15,11,3,0].
//  The array answer is [|0 - 15|,|10 - 11|,|14 - 3|,|22 - 0|] = [15,1,11,22].
//
//
//  Example 2:
//
//  Input: nums = [1]
//  Output: [0]
//  Explanation: The array leftSum is [0] and the array rightSum is [0].
//  The array answer is [|0 - 0|] = [0].
//
//
//
//  Constraints:
//
//
//  	1 <= nums.length <= 1000
//  	1 <= nums[i] <= 10⁵
//

//  // @submit start
impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_vec = vec![0i32; nums.len()];
        let mut left_sum = 0;
        let mut right_vec = vec![0i32; nums.len()];
        let mut right_sum = 0;

        for i in 0..nums.len() {
            let li = i;
            let ri = nums.len() - 1 - i;

            left_vec[li] = left_sum;
            right_vec[ri] = right_sum;

            left_sum += nums[li];
            right_sum += nums[ri];
        }

        left_vec
            .into_iter()
            .zip(right_vec.into_iter())
            .map(|(l, r)| (l - r).abs())
            .collect()
    }
}
//  // @submit end
