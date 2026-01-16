#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 39.517895%

//  You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
//
//  Find the minimum y-coordinate value of a horizontal line such that the total area of the squares above the line equals the total area of the squares below the line.
//
//  Answers within 10-5 of the actual answer will be accepted.
//
//  Note: Squares may overlap. Overlapping areas should be counted multiple times.
//
//
//  Example 1:
//
//
//  Input: squares = [[0,0,1],[2,2,1]]
//
//  Output: 1.00000
//
//  Explanation:
//
//
//
//  Any horizontal line between y = 1 and y = 2 will have 1 square unit above it and 1 square unit below it. The lowest option is 1.
//
//
//  Example 2:
//
//
//  Input: squares = [[0,0,2],[1,1,1]]
//
//  Output: 1.16667
//
//  Explanation:
//
//
//
//  The areas are:
//
//
//  	Below the line: 7/6 * 2 (Red) + 1/6 (Blue) = 15/6 = 2.5.
//  	Above the line: 5/6 * 2 (Red) + 5/6 (Blue) = 15/6 = 2.5.
//
//
//  Since the areas above and below the line are equal, the output is 7/6 = 1.16667.
//
//
//
//  Constraints:
//
//
//  	1 <= squares.length <= 5 * 10⁴
//  	squares[i] = [xi, yi, li]
//  	squares[i].length == 3
//  	0 <= xi, yi <= 10⁹
//  	1 <= li <= 10⁹
//  	The total area of all the squares will not exceed 10¹².
//

//  // @submit start

#[derive(Clone, Copy)]
struct Square {
    bottom: f64,
    area: u64,
    top: f64,
    side_l: f64,
}

impl Square {
    fn new(bottom: i32, side_l: i32) -> Self {
        Self {
            bottom: bottom as f64,
            area: (side_l as u64).pow(2),
            top: (bottom + side_l) as f64,
            side_l: side_l as f64,
        }
    }

    #[inline]
    pub fn get_center(&self) -> f64 {
        self.bottom + self.side_l / 2.0
    }

    #[inline]
    fn bottom_area(&self, div_y: f64) -> f64 {
        let v_side_left = match div_y.total_cmp(&self.top) {
            std::cmp::Ordering::Less => match div_y.total_cmp(&self.bottom) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => 0.0,
                std::cmp::Ordering::Greater => div_y - self.bottom,
            },
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => self.side_l,
        };
        v_side_left * self.side_l
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let squares = Vec::from_iter(squares.iter().map(|sq| Square::new(sq[1], sq[2])));

        assert!(!squares.is_empty());
        if squares.len() == 1 {
            return squares[0].get_center();
        }

        let total_area: u64 = squares.iter().map(|sq| sq.area).sum();
        let b_area_d = total_area as f64 / 2.0;

        let mut min_y = squares.iter().fold(f64::INFINITY, |a, b| a.min(b.bottom));
        let mut max_y = squares.iter().fold(f64::NEG_INFINITY, |a, b| a.max(b.top));
        let mut div_y = (min_y + max_y) / 2.0;

        for _ in 0..50 {
            let b_area: f64 = squares.iter().map(|s| s.bottom_area(div_y)).sum();
            if b_area < b_area_d {
                min_y = div_y;
            } else {
                max_y = div_y;
            }
            div_y = (min_y + max_y) / 2.0;
        }
        div_y
    }
}
//  // @submit end

fn main() {}
