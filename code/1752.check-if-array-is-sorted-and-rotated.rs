//  Category: algorithms
//  Level: Easy
//  Percent: 56.00857%

//  Given an array nums, return true if the array was originally sorted in non-decreasing order, then rotated some number of positions (including zero). Otherwise, return false.
//
//  There may be duplicates in the original array.
//
//  Note: An array A rotated by x positions results in an array B of the same length such that B[i] == A[(i+x) % A.length] for every valid index i.
//
//
//  Example 1:
//
//  Input: nums = [3,4,5,1,2]
//  Output: true
//  Explanation: [1,2,3,4,5] is the original sorted array.
//  You can rotate the array by x = 2 positions to begin on the element of value 3: [3,4,5,1,2].
//
//
//  Example 2:
//
//  Input: nums = [2,1,3,4]
//  Output: false
//  Explanation: There is no sorted array once rotated that can make nums.
//
//
//  Example 3:
//
//  Input: nums = [1,2,3]
//  Output: true
//  Explanation: [1,2,3] is the original sorted array.
//  You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.
//
//
//
//  Constraints:
//
//
//  	1 <= nums.length <= 100
//  	1 <= nums[i] <= 100
//

//  // @submit start
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut iter = nums.into_iter();

        let mut last_n = unsafe { iter.next().unwrap_unchecked() };
        let first = last_n;

        // First non-decreasing batch
        loop {
            match iter.next() {
                Some(n) => {
                    let last_n_c = last_n;
                    last_n = n;

                    if n < last_n_c {
                        break;
                    }
                }
                None => {
                    // Arr was one non-decreasing batch
                    return true;
                }
            }
        }

        while let Some(n) = iter.next() {
            let last_n_c = last_n;
            last_n = n;

            if n < last_n_c {
                return false;
            }
        }

        last_n <= first
    }
}
//  // @submit end
