impl Solution {

    fn find_bound(nums: &Vec<i32>, target: i32, is_first: bool) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut result = -1;

        while l <= r {
            // Calculate mid to avoid overflow
            let mid = l + (r - l) / 2;
            let val = nums[mid as usize];

            if val == target {
                result = mid;
                if is_first {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else if val < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        result
    }


    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let rf = Solution::find_bound(&nums, target, true);
        if rf == -1 {
            return vec![-1, -1];
        }

        let rs = Solution::find_bound(&nums, target, false);

        vec![rf, rs]
    }

}
