//  Category: algorithms
//  Level: Medium
//  Percent: 26.14238%

//  You are given a 0-indexed binary string s and two integers minJump and maxJump. In the beginning, you are standing at index 0, which is equal to '0'. You can move from index i to index j if the following conditions are fulfilled:
//
//
//  	i + minJump <= j <= min(i + maxJump, s.length - 1), and
//  	s[j] == '0'.
//
//
//  Return true if you can reach index s.length - 1 in s, or false otherwise.
//
//
//  Example 1:
//
//  Input: s = "011010", minJump = 2, maxJump = 3
//  Output: true
//  Explanation:
//  In the first step, move from index 0 to index 3.
//  In the second step, move from index 3 to index 5.
//
//
//  Example 2:
//
//  Input: s = "01101110", minJump = 2, maxJump = 3
//  Output: false
//
//
//
//  Constraints:
//
//
//  	2 <= s.length <= 10⁵
//  	s[i] is either '0' or '1'.
//  	s[0] == '0'
//  	1 <= minJump <= maxJump < s.length
//

//  // @submit start
impl Solution {
    const INVALID: u8 = b'1';

    // fn recurse(s: &mut [u8], min_jump: usize, max_jump: usize, pos: usize) -> bool {
    //     s[pos] = Self::INVALID;

    //     if pos == s.len() - 1 {
    //         return true;
    //     }

    //     for i in pos + min_jump..=(pos + max_jump).min(s.len() - 1) {
    //         if s[i] == Self::INVALID {
    //             continue;
    //         }

    //         if Self::recurse(s, min_jump, max_jump, i) {
    //             return true;
    //         }
    //     }

    //     false
    // }

    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;

        let bytes = s.as_bytes();
        let n = bytes.len();

        if bytes[n - 1] == Self::INVALID {
            return false;
        }

        let mut q = std::collections::VecDeque::new();
        q.push_back(0);

        let mut farthest = 0;
        while let Some(i) = q.pop_front() {
            let start = (i + min_jump).max(farthest + 1);
            let end = (i + max_jump).min(n - 1);

            for j in start..=end {
                if bytes[j] == Self::INVALID {
                    continue;
                }
                if j == n - 1 {
                    return true;
                }
                q.push_back(j);
            }

            farthest = farthest.max(end);
        }

        false
    }
}
//  // @submit end
