impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(mut rem: Vec<i32>, ret: &mut Vec<Vec<i32>>, mut act: Vec<i32>) -> Vec<i32> {
            if rem.is_empty() { 
                ret.push(act);
                return rem;
            }

            let rem_len = rem.len();

            for i_next in 0..rem_len {
                let mut act = act.clone();
                let next = rem.swap_remove(i_next);
                act.push(next);
                rem = dfs(rem, ret, act);
                rem.push(next);
                rem.swap(i_next, rem_len - 1);
            }
            rem
        }

        let mut ret = Vec::new();
        dfs(nums, &mut ret, Vec::new());
        ret
    }
}


