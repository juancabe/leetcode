#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 37.67401%

//  You are given the two integers, n and m and two integer arrays, hBars and vBars. The grid has n + 2 horizontal and m + 2 vertical bars, creating 1 x 1 unit cells. The bars are indexed starting from 1.
//
//  You can remove some of the bars in hBars from horizontal bars and some of the bars in vBars from vertical bars. Note that other bars are fixed and cannot be removed.
//
//  Return an integer denoting the maximum area of a square-shaped hole in the grid, after removing some bars (possibly none).
//
//
//  Example 1:
//
//
//
//
//  Input: n = 2, m = 1, hBars = [2,3], vBars = [2]
//
//  |-|-|
//  |-|-|
//  |-|-|
//  |-|-|
//
//  Output: 4
//
//  Explanation:
//
//  |- -|
//  |   |
//  |- -|
//  |- -|
//
//  The left image shows the initial grid formed by the bars. The horizontal bars are [1,2,3,4], and the vertical bars are [1,2,3].
//
//  One way to get the maximum square-shaped hole is by removing horizontal bar 2 and vertical bar 2.
//
//
//  Example 2:
//
//
//
//
//  Input: n = 1, m = 1, hBars = [2], vBars = [2]
//
//  Output: 4
//
//  Explanation:
//
//  To get the maximum square-shaped hole, we remove horizontal bar 2 and vertical bar 2.
//
//
//  Example 3:
//
//
//
//
//  Input: n = 2, m = 3, hBars = [2,3], vBars = [2,4]
//
//  Output: 4
//
//  Explanation:
//
//  One way to get the maximum square-shaped hole is by removing horizontal bar 3, and vertical bar 4.
//
//
//
//  Constraints:
//
//
//  	1 <= n <= 10⁹
//  	1 <= m <= 10⁹
//  	1 <= hBars.length <= 100
//  	2 <= hBars[i] <= n + 1
//  	1 <= vBars.length <= 100
//  	2 <= vBars[i] <= m + 1
//  	All values in hBars are distinct.
//  	All values in vBars are distinct.
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn maximize_square_hole_area(
        n: i32,
        m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        // look for biggest possible square
        // square == 1 h_bar crosses 1 v_bar
        //            (h + 1)         (v + 1) accumulating top
        //            (h + 2)         (v + 2) accumulating top
        // h var h_n crosses v var v_n on (v_n, h_n) (x, y)
        // look for consecutive pairs [(h0, v0), (h1, v1), ..., (h_n, v_n)] where h_i == h_(i - 1) + 1 && v_i == v_(i - 1) + 1
        // the same as
        // look for min(max consecutive bars of h and v)

        struct State {
            last_val: i32,
            streak: usize,
            max_streak: usize,
        }

        impl State {
            fn new(start_val: i32) -> Self {
                Self {
                    last_val: start_val,
                    streak: 1,
                    max_streak: 1,
                }
            }

            fn update(&mut self, curr_val: i32) {
                if self.last_val + 1 == curr_val {
                    self.streak += 1;
                } else {
                    self.streak = 1;
                }
                self.max_streak = self.max_streak.max(self.streak);

                self.last_val = curr_val;
            }
        }

        h_bars.sort();
        v_bars.sort();

        let mut hit = h_bars.into_iter();
        let mut h_state = State::new(hit.next().unwrap());
        for h_bar in hit {
            h_state.update(h_bar);
        }

        let mut vit = v_bars.into_iter();
        let mut v_state = State::new(vit.next().unwrap());
        for v_bar in vit {
            v_state.update(v_bar);
        }

        (v_state.max_streak.min(h_state.max_streak) + 1).pow(2) as i32
    }
}
//  // @submit end

fn main() {}
