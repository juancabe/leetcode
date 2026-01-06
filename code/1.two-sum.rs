#[allow(dead_code)]
struct Solution;

// @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, n) in nums.into_iter().enumerate() {
            if let Some(j) = map.get(&n) {
                return vec![i as i32, *j as i32];
            } else {
                map.insert(target - n, i);
            }
        }

        unreachable!();
    }
}
// @submit end

fn main() {}
