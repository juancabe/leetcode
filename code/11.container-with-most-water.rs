impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_water = 0;

        while left < right {
            let h_left = height[left];
            let h_right = height[right];

            let width = (right - left) as i32;
            let current_height = h_left.min(h_right);

            max_water = max_water.max(current_height * width);

            if h_left < h_right {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_water
    }
}

