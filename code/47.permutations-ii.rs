impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();

        fn backtrack(start: usize, nums: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if start == nums.len() { 
                ret.push(nums.clone());
                return;
            }

            let mut skip = [false; 21];

            for i in start..nums.len() {
                let idx = (nums[i] + 10) as usize;
                if skip[idx] {
                    continue;
                } else {
                    skip[idx] = true;
                }

                nums.swap(start, i);
                backtrack(start + 1, nums, ret);
                nums.swap(start, i);
            }
        }

        backtrack(0, &mut nums, &mut ret);
        ret
    }
}
