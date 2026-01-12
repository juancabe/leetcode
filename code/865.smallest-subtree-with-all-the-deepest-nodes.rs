#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Medium
//  Percent: 72.83524%

//  Given the root of a binary tree, the depth of each node is the shortest distance to the root.
//
//  Return the *smallest subtree* such that it contains all the deepest nodes in the original tree.
//
//  A node is called the *deepest* if it has the largest depth possible among any node in the entire tree.
//
//  The subtree of a node is a tree consisting of that node, plus the set of all descendants of that node.
//
//
//  Example 1:
//
//  Input: root = [3,5,1,6,2,0,8,null,null,7,4]
//  Output: [2,7,4]
//  Explanation: We return the node with value 2, colored in yellow in the diagram.
//  The nodes coloured in blue are the deepest nodes of the tree.
//  Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them, so we return it.
//
//
//  Example 2:
//
//  Input: root = [1]
//  Output: [1]
//  Explanation: The root is the deepest node in the tree.
//
//
//  Example 3:
//
//  Input: root = [0,1,3,null,2]
//  Output: [2]
//  Explanation: The deepest node in the tree is 2, the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
//
//
//
//  Constraints:
//
//
//  	The number of nodes in the tree will be in the range [1, 500].
//  	0 <= Node.val <= 500
//  	The values of the nodes in the tree are unique.
//
//
//
//  Note: This question is the same as 1123: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
//
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

//  // @submit start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[allow(dead_code)]
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn explore(
            p: Option<Rc<RefCell<TreeNode>>>,
            depth: usize,
        ) -> (usize, Option<Rc<RefCell<TreeNode>>>) {
            match p {
                Some(n) => {
                    let (ld, ln) = explore(n.borrow().left.clone(), depth + 1);
                    let (rd, rn) = explore(n.borrow().right.clone(), depth + 1);
                    if ld == rd {
                        (ld, Some(n))
                    } else if rd > ld {
                        (rd, rn)
                    } else {
                        (ld, ln)
                    }
                }
                None => (depth, p),
            }
        }

        let (_d, n) = explore(root, 0);
        n
    }
}
//  // @submit end

fn main() {}
