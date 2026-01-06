impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // search for ordered parts that may contain the target
        fn search_half(nums: &[i32], start: usize, end: usize, target: i32) -> Option<i32> {
            let nums_s = &nums[start..end];

            if nums_s.len() == 0 {
                return None;
            }

            let f = *nums_s.first().unwrap();
            let l = *nums_s.last().unwrap();

            if f > l {
                let mid = start + (end - start) / 2;
                search_half(nums, start, mid, target)
                    .or_else(|| search_half(nums, mid, end, target))
            } else {
                // ordered, binary search
                if f > target || l < target {
                    return None;
                }

                nums_s
                    .binary_search(&target)
                    .ok().map(|n| (n + start) as i32)
            }
        }

        search_half(&nums, 0, nums.len(), target).unwrap_or(-1)
    }
}
