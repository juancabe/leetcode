impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0i32;
        let mut h = (nums.len() - 1) as i32;

        loop {
            let c = l + (h - l) / 2;
            
            if l >= h {
                if nums[l as usize] > target { return l };
                if nums[h as usize] < target { return h + 1 };
                return h;
            }

            if nums[c as usize] == target {
                return c;
            }

            if nums[c as usize] > target {
                h = c - 1;
            } else {
                l = c + 1;
            }

        }
    }
}
