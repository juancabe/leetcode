// fn lex_combinations(mut start: Vec<char>, last_open: usize) -> Vec<String> {
//     let mut ret = Vec::new();
// 
//     let mut found_x = false;
// 
//     for i in last_open..start.len(){
//         if start[i] != 'x' { continue };
// 
//         found_x = true;
//         start[i] = '(';
// 
//         for j in 0..(start.len() - i) / 2 {
//             let close_p_idx = i + 1 + j * 2;
// 
//             if start[close_p_idx] == 'x' {
//                 let mut new_start = start.clone();
//                 new_start[close_p_idx] = ')';
//                 if start[i + 1] == 'x' {
//                     ret.append(&mut lex_combinations(new_start, i));
//                 } else {
//                     for i in i..start.len() {
//                         if start[i] == 'x' {
//                             ret.append(&mut lex_combinations(new_start, i));
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
// 
//         break;
// 
//     }
// 
//     if found_x {
//         ret
//     } else {
//         vec![String::from_iter(start)]
//     }
// 
// }

fn backtrack(
    max: i32,
    open: i32,
    close: i32,
    current: &mut String,
    result: &mut Vec<String>
) {
    // Base Case: We have used all pairs (length is 2 * n)
    if current.len() == (max * 2) as usize {
        result.push(current.clone());
        return;
    }

    // Rule 1: We can add '(' if we haven't reached the limit 'n'
    if open < max {
        current.push('(');
        backtrack(max, open + 1, close, current, result);
        current.pop(); // Backtrack (remove to explore other paths)
    }

    // Rule 2: We can add ')' ONLY if we have more open brackets than closed
    if close < open {
        current.push(')');
        backtrack(max, open, close + 1, current, result);
        current.pop(); // Backtrack
    }
}

impl Solution {
   
    // pub fn generate_parenthesis(n: i32) -> Vec<String> {
    //     let mut start = Vec::from_iter(std::iter::repeat_n('x', n as usize * 2));
    //     let mut vec = lex_combinations(start, 0);

    //     vec.sort();
    //     vec.dedup();
    //     vec
    // }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_string = String::with_capacity((2 * n) as usize);
        
        backtrack(n, 0, 0, &mut current_string, &mut result);
        
        result
    }
    
}
