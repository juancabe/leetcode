// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {

    // [1,2,3,4,5]
    // 1, 2, 0
    // 2, 2, 1
    // 3, 2, 2
    // 4, 2, 3
    // 5, 2, 4
    // _, 2, 5

    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n: usize = n.try_into().unwrap();
        fn explore_list(mut curr: Option<Box<ListNode>>, n: usize, depth: usize)
        -> (Option<Box<ListNode>>, usize) {
            if let Some(mut curr) = curr {
                let (next, len) = explore_list(curr.next.take(), n, depth + 1);
                if n == len - depth {
                    return (next, len);
                }
                curr.next = next;
                return (Some(curr), len);
            } else {
                return (curr, depth)
            }
        }

        let (head, _) = explore_list(head, n, 0);
        head
    }
}
