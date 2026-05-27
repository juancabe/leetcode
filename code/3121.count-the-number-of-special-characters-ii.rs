//  Category: algorithms
//  Level: Medium
//  Percent: 42.959686%

//  You are given a string word. A letter c is called special if it appears both in lowercase and uppercase in word, and every lowercase occurrence of c appears before the first uppercase occurrence of c.
//
//  Return the number of special letters in word.
//
//
//  Example 1:
//
//
//  Input: word = "aaAbcBC"
//
//  Output: 3
//
//  Explanation:
//
//  The special characters are 'a', 'b', and 'c'.
//
//
//  Example 2:
//
//
//  Input: word = "abc"
//
//  Output: 0
//
//  Explanation:
//
//  There are no special characters in word.
//
//
//  Example 3:
//
//
//  Input: word = "AbBCab"
//
//  Output: 0
//
//  Explanation:
//
//  There are no special characters in word.
//
//
//
//  Constraints:
//
//
//  	1 <= word.length <= 2 * 10⁵
//  	word consists of only lowercase and uppercase English letters.
//

//  // @submit start
#[derive(Copy, Clone)]
enum Appearance {
    None,
    Lowercase,
    Both,
    Invalid,
}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut appearances = [Appearance::None; (b'a' - b'A') as usize];

        for b in word.into_bytes() {
            if b < b'a' {
                // uppercase
                let i = (b - b'A') as usize;
                appearances[i] = match appearances[i] {
                    Appearance::Lowercase | Appearance::Both => Appearance::Both,
                    _ => Appearance::Invalid,
                };
            } else {
                // lowercase
                let i = (b - b'a') as usize;
                appearances[i] = match appearances[i] {
                    Appearance::None | Appearance::Lowercase => Appearance::Lowercase,
                    _ => Appearance::Invalid,
                };
            }
        }

        appearances
            .into_iter()
            .filter(|a| matches!(a, Appearance::Both))
            .count() as i32
    }
}
//  // @submit end
