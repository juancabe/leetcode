#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 50.686985%

//  Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
//
//
//  Example 1:
//
//  Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//  Output: [[1,6],[8,10],[15,18]]
//  Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//
//
//  Example 2:
//
//  Input: intervals = [[1,4],[4,5]]
//  Output: [[1,5]]
//  Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
//
//  Example 3:
//
//  Input: intervals = [[4,7],[1,4]]
//  Output: [[1,7]]
//  Explanation: Intervals [1,4] and [4,7] are considered overlapping.
//
//
//
//  Constraints:
//
//
//  	1 <= intervals.length <= 10⁴
//  	intervals[i].length == 2
//  	0 <= starti <= endi <= 10⁴
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|i| *i.first().unwrap());
        let mut it = intervals.into_iter();
        let mut ret = vec![it.next().unwrap()];
        for int in it {
            let f = int[0];
            let s = int[1];
            let rl = ret.len() - 1;

            if f > ret[rl][1] {
                // new interval
                ret.push(int);
            } else if ret[rl][1] < s {
                ret[rl][1] = s;
            }
        }

        ret
    }
}
//  // @submit end

fn main() {}
