#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Hard
//  Percent: 51.598446%

//  The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
//
//  By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
//
//
//  	"123"
//  	"132"
//  	"213"
//  	"231"
//  	"312"
//  	"321"
//
//
//  Given n and k, return the kth permutation sequence.
//
//
//  Example 1:
//  Input: n = 3, k = 3
//  Output: "213"
//  Example 2:
//  Input: n = 4, k = 9
//  Output: "2314"
//  Example 3:
//  Input: n = 3, k = 1
//  Output: "123"
//
//
//  Constraints:
//
//
//  	1 <= n <= 9
//  	1 <= k <= n!
//

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums: Vec<char> = (1..=n as u8).map(|d| (d + b'0') as char).collect();

        let mut k = k as usize - 1;
        let mut result = String::new();

        let mut fact = vec![1; n as usize];
        for i in 1..n as usize {
            fact[i] = fact[i - 1] * i;
        }

        for i in (0..n as usize).rev() {
            let block_size = fact[i];
            let index = k / block_size;
            result.push(nums.remove(index));
            k %= block_size;
        }

        result
    }
}
//  // @submit end

fn main() {}
