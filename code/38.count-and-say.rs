impl Solution {
//     pub fn count_and_say(n: i32) -> String {
//         if n == 1 {
//             return "1".to_string();
//         }
// 
//         let prev = Self::count_and_say(n - 1);
// 
//         let mut ret = String::with_capacity(prev.len() * 2);
//         let mut iter = prev.chars().peekable();
// 
//         let mut curr_c = iter.next().unwrap();
//         let mut count = 1;
//         for c in iter {
//             if curr_c == c {
//                 count += 1;
//             } else {
//                 ret.push_str(&count.to_string());
//                 ret.push(curr_c);
//                 curr_c = c;
//                 count = 1;
//             }
//         }
//         ret.push_str(&count.to_string());
//         ret.push(curr_c);
// 
//         ret
// 
//     }

    pub fn count_and_say(n: i32) -> String {

        let mut prev = "1".to_string();
        let mut ret = String::with_capacity(2 << (n / 2));
        let mut use1 = true;

        for i in 1..n {
            let mut iter = prev.chars().peekable();

            let mut curr_c = iter.next().unwrap();
            let mut count = 1;
            for c in iter {
                if curr_c == c {
                    count += 1;
                } else {
                    ret.push_str(&count.to_string());
                    ret.push(curr_c);
                    curr_c = c;
                    count = 1;
                }
            }
            ret.push_str(&count.to_string());
            ret.push(curr_c);

            prev.clear();
            std::mem::swap(&mut prev, &mut ret);
        }

        prev

    }
}
