#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Hard
//  Percent: 23.20749%

//  You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
//
//  Find the minimum y-coordinate value of a horizontal line such that the total area covered by squares above the line equals the total area covered by squares below the line.
//
//  Answers within 10-5 of the actual answer will be accepted.
//
//  Note: Squares may overlap. Overlapping areas should be counted only once in this version.
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
//  Any horizontal line between y = 1 and y = 2 results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
//
//
//  Example 2:
//
//
//  Input: squares = [[0,0,2],[1,1,1]]
//
//  Output: 1.00000
//
//  Explanation:
//
//
//
//  Since the blue square overlaps with the red square, it will not be counted again. Thus, the line y = 1 splits the squares into two equal parts.
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
//  	The total area of all the squares will not exceed 10¹⁵.
//

//  // @submit start

struct Square {
    // bottom x
    x: f64,
    // bottom y
    y: f64,
    dx: f64,
    dy: f64,
}

impl Square {
    fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self { x, y, dx, dy }
    }

    #[inline]
    fn max_x(&self) -> f64 {
        self.x + self.dx
    }

    #[inline]
    fn max_y(&self) -> f64 {
        self.y + self.dy
    }

    fn add_splits(&self, other: &Square, to_add_squares: &mut Vec<Square>) -> usize {
        todo!()
    }

    #[inline]
    fn bottom_area(&self, div_y: f64) -> f64 {
        let v_side_left = match div_y.total_cmp(&self.max_y()) {
            std::cmp::Ordering::Less => match div_y.total_cmp(&self.y) {
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => 0.0,
                std::cmp::Ordering::Greater => div_y - self.y,
            },
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => self.dy,
        };
        v_side_left * self.dx
    }

    fn add_from_sq_and_right_sqs(
        sq: [f64; 3],
        right: impl Iterator<Item = [f64; 3]>,
        pool: &mut Vec<Square>,
        holds_ret_arena: &mut Vec<Square>,
        swap_arena: &mut Vec<Square>,
    ) {
        use std::mem;

        let [x, y, len] = sq;
        holds_ret_arena.clear();
        holds_ret_arena.push(Square::new(x, y, len, len));

        for [ox, oy, ol] in right {
            let new_sq = Square::new(ox, oy, ol, ol);
            swap_arena.clear();
            for sq in holds_ret_arena.iter() {
                sq.add_splits(&new_sq, swap_arena);
            }
            mem::swap(holds_ret_arena, swap_arena);
        }

        pool.append(holds_ret_arena);
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut squares_f = Vec::new();
        let mut arena0 = Vec::new();
        let mut arena1 = Vec::new();

        for i in 0..squares.len() {
            let sq = &squares[i];
            let sq = [sq[0] as f64, sq[1] as f64, sq[2] as f64];
            Square::add_from_sq_and_right_sqs(
                sq,
                squares[i + 1..]
                    .iter()
                    .map(|sq| [sq[0] as f64, sq[1] as f64, sq[2] as f64]),
                &mut squares_f,
                &mut arena0,
                &mut arena1,
            );
        }

        let squares = squares_f;

        let mut max_y = squares
            .iter()
            .max_by(|sq0, sq1| sq0.y.total_cmp(&sq1.y))
            .unwrap()
            .y;

        let mut min_y = squares
            .iter()
            .min_by(|sq0, sq1| sq0.y.total_cmp(&sq1.y))
            .unwrap()
            .y;

        let total_area: f64 = squares.iter().map(|sq| sq.bottom_area(max_y + 1.0)).sum();
        let desired_area = total_area / 2.0;

        for _ in 0..100 {
            let mid = (max_y + min_y) / 2.0;
            let area: f64 = squares.iter().map(|sq| sq.bottom_area(mid)).sum();
            if area < desired_area {
                min_y = mid;
            } else {
                max_y = mid;
            }
        }

        (max_y + min_y) / 2.0
    }
}
//  // @submit end

fn main() {}
