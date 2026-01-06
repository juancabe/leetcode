use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = nums.len();
        if n < 4 { return result; }

        let target = target as i64;
        let mut nums: Vec<i64> = nums.into_iter().map(|n| n as i64).collect();
        nums.sort_unstable();

        for i in 0..n - 3 {
            if i > 0 && nums[i] == nums[i - 1] { continue; }

            let min_sum_i = nums[i] + nums[i+1] + nums[i+2] + nums[i+3];
            if min_sum_i > target { break; } // Smallest possible sum is too big -> STOP

            let max_sum_i = nums[i] + nums[n-1] + nums[n-2] + nums[n-3];
            if max_sum_i < target { continue; } // Biggest possible sum is too small -> Next 'i'


            for j in i + 1..n - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] { continue; }

                let min_sum_j = nums[i] + nums[j] + nums[j+1] + nums[j+2];
                if min_sum_j > target { break; } // Smallest sum for j is too big -> Next 'i'

                let max_sum_j = nums[i] + nums[j] + nums[n-1] + nums[n-2];
                if max_sum_j < target { continue; } // Biggest sum for j is too small -> Next 'j'


                let mut left = j + 1;
                let mut right = n - 1;

                while left < right {
                    let sum = nums[i] + nums[j] + nums[left] + nums[right];

                    match sum.cmp(&(target)) {
                        Ordering::Equal => {
                            result.push(vec![nums[i] as i32, nums[j] as i32, nums[left] as i32, nums[right] as i32]);
                            left += 1;
                            right -= 1;
                            while left < right && nums[left] == nums[left - 1] { left += 1; }
                            while left < right && nums[right] == nums[right + 1] { right -= 1; }
                        },
                        Ordering::Less => left += 1,
                        Ordering::Greater => right -= 1,
                    }
                }
            }
        }

        result
    }
}
