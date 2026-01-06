use std::cmp::Ordering;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let len = nums.len();

        let mut nearest: Option<i32> = None;

        for i in 0..len - 2 {
            let mut right = len - 1;
            let mut left = i + 1;

            while right > left {
                let sum = nums[i] + nums[right] + nums[left];

                nearest = nearest
                    .map_or(Some(sum), |prev| {
                        if sum.abs_diff(target) < prev.abs_diff(target) {
                            Some(sum)
                        } else {
                            Some(prev)
                        }
                    });

                match sum.cmp(&target) {
                    Ordering::Equal => {
                        return target;
                    },
                    Ordering::Less => {
                        // need higher sum, move left
                        left += 1;
                    },
                    Ordering::Greater => {
                        // need lower sum, move right
                        right -= 1;
                    }
                }
            }

        }

        nearest.unwrap()

    }
}
