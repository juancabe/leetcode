//  Category: algorithms
//  Level: Medium
//  Percent: 66.87334%

//  Given an array of non-negative integers arr, you are initially positioned at start index of the array. When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach any index with value 0.
//
//  Notice that you can not jump outside of the array at any time.
//
//
//  Example 1:
//
//  Input: arr = [4,2,3,0,3,1,2], start = 5
//  Output: true
//  Explanation:
//  All possible ways to reach at index 3 with value 0 are:
//  index 5 -> index 4 -> index 1 -> index 3
//  index 5 -> index 6 -> index 4 -> index 1 -> index 3
//
//
//  Example 2:
//
//  Input: arr = [4,2,3,0,3,1,2], start = 0
//  Output: true
//  Explanation:
//  One possible way to reach at index 3 with value 0 is:
//  index 0 -> index 4 -> index 1 -> index 3
//
//
//  Example 3:
//
//  Input: arr = [3,0,2,1,2], start = 2
//  Output: false
//  Explanation: There is no way to reach at index 1 with value 0.
//
//
//
//  Constraints:
//
//
//  	1 <= arr.length <= 5 * 10⁴
//  	0 <= arr[i] < arr.length
//  	0 <= start < arr.length
//

//  // @submit start
use std::collections::HashSet;

impl Solution {

    fn dfs(start: usize, arr: &[i32], explored: &mut HashSet<usize>) -> bool {
        if arr[start] == 0 {
            return true;
        }
        explored.insert(start);

        if let Some(l) = start.checked_sub(arr[start] as usize) && !explored.contains(&l) && Self::dfs(l, arr, explored) {
            return true;  
        }

        let r = start + arr[start] as usize;
        if r < arr.len() && !explored.contains(&r) && Self::dfs(r, arr, explored) {
            return true;  
        }

        return false;
    }

    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut explored = HashSet::<usize>::new();
        Self::dfs(start.try_into().unwrap(), &arr, &mut explored)
    }
}
//  // @submit end
