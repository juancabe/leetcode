//  Category: algorithms
//  Level: Medium
//  Percent: 81.68667%

//  You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,
//
//
//  	If isLefti == 1, then childi is the left child of parenti.
//  	If isLefti == 0, then childi is the right child of parenti.
//
//
//  Construct the binary tree described by descriptions and return its root.
//
//  The test cases will be generated such that the binary tree is valid.
//
//
//  Example 1:
//
//  Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
//  Output: [50,20,80,15,17,19]
//  Explanation: The root node is the node with value 50 since it has no parent.
//  The resulting binary tree is shown in the diagram.
//
//
//  Example 2:
//
//  Input: descriptions = [[1,2,1],[2,3,0],[3,4,1]]
//  Output: [1,2,null,null,3,4]
//  Explanation: The root node is the node with value 1 since it has no parent.
//  The resulting binary tree is shown in the diagram.
//
//
//
//  Constraints:
//
//
//  	1 <= descriptions.length <= 10⁴
//  	descriptions[i].length == 3
//  	1 <= parenti, childi <= 10⁵
//  	0 <= isLefti <= 1
//  	The binary tree described by descriptions is valid.
//

//  // @submit start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::VacantEntry;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        //                             node id,      node              has_p
        let mut parsed_nodes = HashMap::<i32, (Rc<RefCell<TreeNode>>, bool)>::new();

        for [p, c, l] in descriptions.into_iter().map(|d| [d[0], d[1], d[2]]) {
            let child = Rc::new(RefCell::new(TreeNode::new(c)));

            let child = match parsed_nodes.entry(c) {
                Entry::Occupied(mut oe) => {
                    let (c, has_p) = oe.get_mut();
                    *has_p = true;
                    Rc::clone(c)
                }
                Entry::Vacant(v) => {
                    v.insert((Rc::clone(&child), true));
                    child
                }
            };

            match parsed_nodes.entry(p) {
                Entry::Occupied(mut oe) => {
                    let (node, has_p) = oe.get_mut();
                    if l == 1 {
                        node.borrow_mut().left = Some(child);
                    } else {
                        node.borrow_mut().right = Some(child);
                    }
                }
                Entry::Vacant(v) => {
                    let mut p_node = TreeNode::new(p);
                    if l == 1 {
                        p_node.left = Some(child);
                    } else {
                        p_node.right = Some(child);
                    }

                    let p = Rc::new(RefCell::new(p_node));
                    v.insert((p, false));
                }
            }
        }

        parsed_nodes
            .into_iter()
            .map(|(_, v)| v)
            .filter_map(|(n, hp)| if hp { None } else { Some(n) })
            .next()
    }
}
//  // @submit end
