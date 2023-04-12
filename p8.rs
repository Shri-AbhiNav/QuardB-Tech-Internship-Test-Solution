// Given a binary tree, implement a function that returns the maximum depth of the tree.

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + std::cmp::max(left_depth, right_depth)
        }
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    node2.left = Some(Box::new(node3));
    root.right = Some(Box::new(node2));

    println!("Maximum depth of the tree: {}", max_depth(Some(Box::new(root))));
}
