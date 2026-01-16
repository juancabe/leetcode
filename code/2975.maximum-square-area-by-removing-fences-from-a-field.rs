#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 24.932999%

//  There is a large (m - 1) x (n - 1) rectangular field with corners at (1, 1) and (m, n) containing some horizontal and vertical fences given in arrays hFences and vFences respectively.
//
//  Horizontal fences are from the coordinates (hFences[i], 1) to (hFences[i], n) and vertical fences are from the coordinates (1, vFences[i]) to (m, vFences[i]).
//
//  Return the maximum area of a square field that can be formed by removing some fences (possibly none) or -1 if it is impossible to make a square field.
//
//  Since the answer may be large, return it modulo 10^9 + 7.
//
//  Note: The field is surrounded by two horizontal fences from the coordinates (1, 1) to (1, n) and (m, 1) to (m, n) and two vertical fences from the coordinates (1, 1) to (m, 1) and (1, n) to (m, n). These fences cannot be removed.
//
//
//  Example 1:
//
//
//
//  Input: m = 4, n = 3, hFences = [2,3], vFences = [2]
//  Output: 4
//  Explanation: Removing the horizontal fence at 2 and the vertical fence at 2 will give a square field of area 4.
//
//
//  Example 2:
//
//
//
//  Input: m = 6, n = 7, hFences = [2], vFences = [4]
//  Output: -1
//  Explanation: It can be proved that there is no way to create a square field by removing fences.
//
//
//
//  Constraints:
//
//
//  	3 <= m, n <= 10⁹
//  	1 <= hFences.length, vFences.length <= 600
//  	1 < hFences[i] < m
//  	1 < vFences[i] < n
//  	hFences and vFences are unique.
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        // square:  4 fences (f)
        //          2 vertical fences (vf)   vf_0, vf_1 / |vf_0 - vf_1| = dv
        //          2 horizontal fences (hf) hf_0, hf_1 / |hf_0 - hf_1| = dh
        //          square if dv == dh

        h_fences.push(1);
        h_fences.push(m);
        v_fences.push(1);
        v_fences.push(n);

        use std::collections::HashSet;

        let mut h_dxs = HashSet::with_capacity(h_fences.len().pow(2));

        for i in 0..h_fences.len() {
            for j in i + 1..h_fences.len() {
                h_dxs.insert(h_fences[i].abs_diff(h_fences[j]));
            }
        }

        let mut max_dx: i32 = -1;
        for i in 0..v_fences.len() {
            for j in i + 1..v_fences.len() {
                let dx = v_fences[i].abs_diff(v_fences[j]);
                if h_dxs.contains(&dx) {
                    max_dx = max_dx.max(dx as i32)
                }
            }
        }

        if max_dx > 0 {
            ((max_dx as u64).pow(2) % (1_000_000_000 + 7)) as i32
        } else {
            -1
        }
    }
}
//  // @submit end

fn main() {}
