#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 66.206505%

//  Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.
//
//
//  Example 1:
//
//  Input: s1 = "sea", s2 = "eat"
//  Output: 231
//  Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
//  Deleting "t" from "eat" adds 116 to the sum.
//  At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
//
//
//  Example 2:
//
//  Input: s1 = "delete", s2 = "leet"
//  Output: 403
//  Explanation: Deleting "dee" from "delete" to turn the string into "let",
//  adds 100[d] + 101[e] + 101[e] to the sum.
//  Deleting "e" from "leet" adds 101[e] to the sum.
//  At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
//  If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
//
//
//
//  Constraints:
//
//
//  	1 <= s1.length, s2.length <= 1000
//  	s1 and s2 consist of lowercase English letters.
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut mat: Vec<Vec<u64>> = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        let b1: Vec<u64> = s1.bytes().map(|b| b as u64).collect();
        let b2: Vec<u64> = s2.bytes().map(|b| b as u64).collect();

        for i in 1..=s1.len() {
            mat[i][0] = b1[i - 1] + mat[i - 1][0];
        }
        for i in 1..=s2.len() {
            mat[0][i] = b2[i - 1] + mat[0][i - 1];
        }

        for i1 in 0..b1.len() {
            let c1 = b1[i1];
            for i2 in 0..b2.len() {
                let c2 = b2[i2];
                mat[i1 + 1][i2 + 1] = if c1 == c2 {
                    mat[i1][i2]
                } else {
                    (c1 + mat[i1][i2 + 1]).min(c2 + mat[i1 + 1][i2])
                };
            }
        }

        mat[s1.len()][s2.len()] as i32
    }
}
//  // @submit end

fn main() {}
