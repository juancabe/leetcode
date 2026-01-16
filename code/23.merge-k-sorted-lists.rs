#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Hard
//  Percent: 58.37038%

//  You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//
//  Merge all the linked-lists into one sorted linked-list and return it.
//
//
//  Example 1:
//
//  Input: lists = [[1,4,5],[1,3,4],[2,6]]
//  Output: [1,1,2,3,4,4,5,6]
//  Explanation: The linked-lists are:
//  [
//    1->4->5,
//    1->3->4,
//    2->6
//  ]
//  merging them into one sorted linked list:
//  1->1->2->3->4->4->5->6
//
//
//  Example 2:
//
//  Input: lists = []
//  Output: []
//
//
//  Example 3:
//
//  Input: lists = [[]]
//  Output: []
//
//
//
//  Constraints:
//
//
//  	k == lists.length
//  	0 <= k <= 10⁴
//  	0 <= lists[i].length <= 500
//  	-10⁴ <= lists[i][j] <= 10⁴
//  	lists[i] is sorted in ascending order.
//  	The sum of lists[i].length will not exceed 10⁴.
//

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//  // @submit start
impl Solution {
    #[allow(dead_code)]
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut root: Option<Box<ListNode>> = None;

        for mut list in lists {
            let mut ret_head = root.as_mut();
            let mut curr_l_head = list.as_mut();
            while let Some(ref ret_node) = ret_head
                && let Some(ref curr_l_node) = curr_l_head
            {
                let next_val: i32 = ret_node.next.as_ref().map(|n| n.val).unwrap_or(i32::MAX);
                let curr_val = ret_node.val;
                if curr_l_node.val <= next_val && curr_l_node.val > curr_val {
                    todo!()
                }
            }

            if let Some(curr_l_node) = curr_l_head {
                todo!()
            }
        }

        root
    }
}
//  // @submit end

fn main() {}
