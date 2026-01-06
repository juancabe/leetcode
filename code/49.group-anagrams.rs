impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        use std::collections::hash_map::Entry;

        type Anagram = [u8; 26];
        let mut ret: HashMap<Anagram, Vec<String>> = HashMap::new();
        let a_val = 'a' as usize;

        for s in strs.into_iter() {
            let mut c_anagram: Anagram = [0; 26];

            for c in s.chars() {
                let val = c as usize - a_val;
                c_anagram[val] += 1;
            }

            match ret.entry(c_anagram) {
                Entry::Occupied(mut e) => e.get_mut().push(s),
                Entry::Vacant(e) => { e.insert(vec![s]); },
            };
        }

        ret.into_values().collect()

    }
}
