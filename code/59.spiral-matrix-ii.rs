#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 74.3622%

//  Given a positive integer n, generate an n x n matrix filled with elements from 1 to n² in spiral order.
//
//
//  Example 1:
//
//  Input: n = 3
//  Output: [[1,2,3]
//          ,[8,9,4]
//          ,[7,6,5]]
//
//  Input: n = 4
//  Output: [[1,2,3,4]
//          [12,9,4,5]
//          [11,9,4,6]
//          [10,9,8,7]]
//
//  x x x x x x 7
//  5 x x x x 5 x
//  x 3 x x 3 x x
//  x x 1 1 x x x
//  x x 2 x 2 x x
//  x 4 x x x 4 x
//  6 x x x x x 6
//
//  n       r
//  n - 1   d
//  n - 1   l
//  n - 2   u
//  n - 2   r
//  n - 3   d
//  n - 3   l
//  n - 4
//
//
//  Example 2:
//
//  Input: n = 1
//  Output: [[1]]
//
//
//
//  Constraints:
//
//
//  	1 <= n <= 20
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        use std::convert::TryInto;
        use std::iter::FromIterator;
        use std::iter::repeat_n;

        if n == 1 {
            return vec![vec![1]];
        };

        let n: usize = n.try_into().unwrap();
        let mut mat = Vec::from_iter(repeat_n(Vec::from_iter(repeat_n(0, n)), n));
        let mut movement = n;
        let mut acc: i32 = 1;
        let mut i = 0;
        let mut j = 0;
        for _ in 0..movement {
            mat[i][j] = acc;
            acc += 1;
            j += 1;
        }
        j -= 1;

        loop {
            movement -= 1;
            // down
            for _ in 0..movement {
                i += 1;
                mat[i][j] = acc;
                acc += 1;
            }
            // left
            for _ in 0..movement {
                j -= 1;
                mat[i][j] = acc;
                acc += 1;
            }
            movement -= 1;
            // up
            for _ in 0..movement {
                i -= 1;
                mat[i][j] = acc;
                acc += 1;
            }
            // right
            for _ in 0..movement {
                j += 1;
                mat[i][j] = acc;
                acc += 1;
            }
            if movement <= 1 {
                break;
            }
        }

        mat
    }
}
//  // @submit end

fn main() {
    let res = Solution::generate_matrix(3);
    println!("3: {res:?}");
    let res = Solution::generate_matrix(2);
    println!("2: {res:?}");
}
