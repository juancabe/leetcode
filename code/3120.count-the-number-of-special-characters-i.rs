//  Category: algorithms
//  Level: Easy
//  Percent: 67.02278%

//  You are given a string word. A letter is called special if it appears both in lowercase and uppercase in word.
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
//  The special characters in word are 'a', 'b', and 'c'.
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
//  No character in word appears in uppercase.
//
//
//  Example 3:
//
//
//  Input: word = "abBCab"
//
//  Output: 1
//
//  Explanation:
//
//  The only special character in word is 'b'.
//
//
//
//  Constraints:
//
//
//  	1 <= word.length <= 50
//  	word consists of only lowercase and uppercase English letters.
//

//  // @submit start
#[derive(Clone, Copy)]
enum Appearance {
    None,
    Both,
    Lowercase,
    Uppercase,
}

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut appearances = [Appearance::None; (b'z' - b'a') as usize + 1];

        for l in word.as_bytes() {
            let mut l = *l;
            let upper = if l > b'Z' {
                l -= (b'a' - b'A');
                true
            } else {
                false
            };

            let i = (l - b'A') as usize;
            match appearances[i] {
                Appearance::None => {
                    appearances[i] = if upper {
                        Appearance::Uppercase
                    } else {
                        Appearance::Lowercase
                    };
                }
                Appearance::Both => (),
                Appearance::Lowercase => {
                    if upper {
                        appearances[i] = Appearance::Both;
                    }
                }
                Appearance::Uppercase => {
                    if !upper {
                        appearances[i] = Appearance::Both;
                    }
                }
            }
        }

        appearances
            .into_iter()
            .filter(|a| matches!(a, Appearance::Both))
            .count() as i32
    }
}
//  // @submit end
