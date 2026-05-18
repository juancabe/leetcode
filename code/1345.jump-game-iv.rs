//  Category: algorithms
//  Level: Hard
//  Percent: 46.31638%

//  Given an array of integers arr, you are initially positioned at the first index of the array.
//
//  In one step you can jump from index i to index:
//
//
//  	i + 1 where: i + 1 < arr.length.
//  	i - 1 where: i - 1 >= 0.
//  	j where: arr[i] == arr[j] and i != j.
//
//
//  Return the minimum number of steps to reach the last index of the array.
//
//  Notice that you can not jump outside of the array at any time.
//
//
//  Example 1:
//
//  Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
//  Output: 3
//  Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
//
//
//  Example 2:
//
//  Input: arr = [7]
//  Output: 0
//  Explanation: Start index is the last index. You do not need to jump.
//
//
//  Example 3:
//
//  Input: arr = [7,6,9,6,9,6,9,7]
//  Output: 1
//  Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
//
//
//
//  Constraints:
//
//
//  	1 <= arr.length <= 5 * 10⁴
//  	-10⁸ <= arr[i] <= 10⁸
//

//  // @submit start
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    fn maybe_queue(
        new_pos: usize,
        queue: &mut VecDeque<(usize, u32)>,
        explored: &mut [bool],
        jumps_to_queue: u32,
    ) {
        if explored[new_pos] {
            return;
        }

        queue.push_back((new_pos, jumps_to_queue));
        explored[new_pos] = true;
    }

    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        //                             num   positions
        let mut duplicates = HashMap::<i32, Vec<usize>>::new();
        for (pos, i) in arr.iter().enumerate() {
            duplicates
                .entry(*i)
                .and_modify(|v| v.push(pos))
                .or_insert(vec![pos]);
        }

        let mut explored = vec![false; arr.len()];
        let mut queue = VecDeque::new();
        queue.push_back((0usize, 0u32));

        let none: Vec<usize> = vec![];
        while let Some((pos, jumps)) = queue.pop_front() {
            if pos == arr.len() - 1 {
                return jumps as i32;
            }

            let mut recv_jumps = u16::MAX;
            if let Some(l) = pos.checked_sub(1) {
                Self::maybe_queue(l, &mut queue, &mut explored, jumps + 1)
            }

            let r = pos + 1;
            if r <= arr.len() - 1 {
                Self::maybe_queue(r, &mut queue, &mut explored, jumps + 1)
            }

            if let Some(dps) = duplicates.get(&arr[pos]) {
                for d in dps {
                    Self::maybe_queue(*d, &mut queue, &mut explored, jumps + 1);
                }

                duplicates.remove(&arr[pos]);
            }
        }

        unreachable!()
    }
}
//  // @submit end
