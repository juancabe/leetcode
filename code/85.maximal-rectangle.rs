#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Hard
//  Percent: 55.532944%

//  Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
//
//
//  Example 1:
//
//  Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
//  Output: 6
//  Explanation: The maximal rectangle is shown in the above picture.
//
//
//  Example 2:
//
//  Input: matrix = [["0"]]
//  Output: 0
//
//
//  Example 3:
//
//  Input: matrix = [["1"]]
//  Output: 1
//
//
//
//  Constraints:
//
//
//  	rows == matrix.length
//  	cols == matrix[i].length
//  	1 <= rows, cols <= 200
//  	matrix[i][j] is '0' or '1'.
//
//
//
//

// - - - - -
// - 1 1 1 1
// - 1 2 2 4
// - 1 2 9 - <-- (4 - 2 + 1) + 9
// - - - - -
// - - - - -
// 1 0 1 1 1
// 1 0 1 1 1
// 1 1 1 4 9 <-- (6 - 4 + 1) + 6
// 1 0 1 9 0
//
//
//
// 1 0 1 0 0
// 1 0 1 0 0
// 1 0 1 0 0
// 1 0 1 0 0
// 0 1 6 1 1
// 1 2 1 1 1
// 1 0 0 1 0↩

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights = vec![vec![0; matrix[0].len()]; matrix.len()];
        for j in 0..matrix[0].len() {
            heights[0][j] = matrix[0][j] as u32 - '0' as u32;
        }

        for j in 0..matrix[0].len() {
            for i in 1..matrix.len() {
                if matrix[i][j] == '0' {
                    heights[i][j] = 0;
                } else {
                    heights[i][j] = heights[i - 1][j] + 1;
                }
            }
        }

        fn largest_rectangle_area(heights: &[u32]) -> u32 {
            let mut inc_indices: Vec<usize> = Vec::new();
            let mut max_area = 0;
            let mut i = 0;

            while i <= heights.len() {
                let current_h = if i == heights.len() { 0 } else { heights[i] };

                if inc_indices.is_empty() || current_h >= heights[*inc_indices.last().unwrap()] {
                    inc_indices.push(i);
                    i += 1;
                } else {
                    let h = heights[inc_indices.pop().unwrap()];
                    let w = if inc_indices.is_empty() {
                        i
                    } else {
                        i - inc_indices.last().unwrap() - 1
                    };
                    max_area = max_area.max(h * w as u32);
                }
            }
            max_area
        }

        let mut max = 0;

        for row in heights {
            let current_area = largest_rectangle_area(&row);
            max = max.max(current_area);
        }

        max as i32
    }
}
//  // @submit end

fn main() {}
