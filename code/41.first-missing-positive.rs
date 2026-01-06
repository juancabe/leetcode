impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // for nums len n, the first missing positive must be within 1 ... n + 1
        // 1 Map each nums[i] to wether i + 1 is present or not in nums

        const PRESENT: i32 = -1;
        const FREE: i32 = -2;

        for n in &mut nums {
            if *n <= 0 {
                *n = FREE;
            }
        }

        for i in 0..nums.len() {
            let n = nums[i] - 1;
            if n != PRESENT - 1 {
                nums[i] = FREE;
            }

            if n >= 0 && n < nums.len() as i32 {
                let mut to_set_active = n;
                loop {
                    let temp = nums[to_set_active as usize] - 1;
                    nums[to_set_active as usize] = PRESENT;

                    if temp >= 0 && temp < nums.len() as i32 {
                        to_set_active = temp;
                    } else {
                        break;
                    }
                }
            }
        }

        for (i, n) in nums.iter().enumerate() {
            if *n != PRESENT {
                return i as i32 + 1
            }
        }

        return nums.len() as i32 + 1
        
    }
}
