 impl Solution {
     pub fn reverse(x: i32) -> i32 {
         let max = "2147483648";
         let parsed = x.to_string();
 
         let slice = if x < 0 { &parsed[1..] }  else  { &parsed };
         if slice.len() == max.len() && slice.chars().rev().gt(max.chars()) {
             return 0;
         }
         
         let mut parsed = parsed.into_bytes();
         parsed.reverse();
         if x < 0 { parsed.pop().unwrap(); }
         let parsed: String = unsafe {
             String::from_utf8_unchecked(parsed)
         };
 
         let parsed: i32 = parsed.parse().unwrap();
         
         if x < 0 { -parsed } else { parsed }
 
     }
 }


