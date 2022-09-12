//!
//! A good bit of the design for this came from 
//! Free Code Camp's video on binary tree algorithms
//! that I rewrote in Rust. It has great explanations
//! and if anyone would like to watch it can be found
//! at https://www.youtube.com/watch?v=fAAZixBzIAI
//! 



pub struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    const fn new(val: i32) -> Node {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn depth_first(root: Node) -> Vec<i32> {
    let mut stack = vec![root];
    let mut resvec = vec![];
    while !stack.is_empty() {
        let current = match stack.pop() {
            Some(node) => node,
            // loop breaks because stack is empty
            // and all nodes have been traversed
            None => break,
        };

        resvec.push(current.val);

        if current.right.is_some() { stack.push(*current.right.unwrap()); }
        if current.left.is_some() { stack.push(*current.left.unwrap()); }

    }
    resvec
}

fn depth_first_recursive(root: Node) -> Vec<i32> {
    let mut vec = vec![];

    let mut left = match root.left {
        Some(left_node) => depth_first_recursive(*left_node),
        None => vec![],
    };

    let mut right = match root.right {
        Some(right_node) => depth_first_recursive(*right_node),
        None => vec![],
    };

    vec.push(root.val);
    vec.append(&mut left);
    vec.append(&mut right);

    
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_first_imperative_test() {
        let leaf1 = Node::new(4);
        let leaf2 = Node::new(3);
        let node = Node {val: 2, left: Some(Box::new(leaf1)), right: None};
        let root = Node {val: 1, left: Some(Box::new(node)), right: Some(Box::new(leaf2)) };

        let res = depth_first(root);
        // tree should look like this
        //      1
        //     / \
        //    2   3
        //  /
        // 4
        assert_eq!(res, vec![1, 2, 4, 3]);
    }

    #[test]
    fn depth_first_recursive_test() {
        let leaf1 = Node::new(4);
        let leaf2 = Node::new(3);
        let leaf3 = Node::new(5);
        let node = Node {val: 2, left: Some(Box::new(leaf1)), right: Some(Box::new(leaf3))};
        let root = Node {val: 1, left: Some(Box::new(node)), right: Some(Box::new(leaf2)) };

        let res = depth_first_recursive(root);
        // tree should look like this
        //      1
        //     / \
        //    2   3
        //  /  \
        // 4    5
        assert_eq!(res, vec![1, 2, 4, 5, 3]);
    }
}

