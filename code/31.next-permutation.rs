impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut p = None;

        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                p = Some(i - 1);
                break;
            }
        }

        let p1 = if let Some(p) = p {
            p
        } else {
            nums.reverse();
            return;
        };

        let mut p2 = nums.len() - 1;
        while nums[p2] <= nums[p1] {
            p2 -= 1;
        }

        nums.swap(p1, p2);
        nums[p1 + 1..].reverse();
    }
}
