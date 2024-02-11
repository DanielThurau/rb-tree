use crate::{
    node::{Node, NodeColor},
    tree::Tree,
};
use proptest::prelude::*;
use std::{cell::RefCell, cmp::min, rc::Rc};

impl<T> Tree<T> {
    fn construct(root: Rc<RefCell<Node<T>>>, sentinel: Rc<RefCell<Node<T>>>) -> Tree<T> {
        Self { root, sentinel }
    }
}

#[test]
fn test_left_rotate() {
    // Build a Tree that looks like this:
    //
    //               2
    //             /   \
    //            1      4
    //                  /  \
    //                 3    5
    //
    let sentinel = Rc::new(RefCell::new(Node::new_sentinel()));
    let mut a = Node::new(1);
    a.set_left_child(sentinel.clone());
    a.set_right_child(sentinel.clone());
    let mut b = Node::new(3);
    b.set_left_child(sentinel.clone());
    b.set_right_child(sentinel.clone());
    let mut c = Node::new(5);
    c.set_left_child(sentinel.clone());
    c.set_right_child(sentinel.clone());
    let y = Rc::new(RefCell::new(Node::new(4)));
    let x = Rc::new(RefCell::new(Node::new(2)));
    c.set_parent(y.clone());
    b.set_parent(y.clone());
    a.set_parent(x.clone());
    y.borrow_mut().set_left_child(Rc::new(RefCell::new(b)));
    y.borrow_mut().set_right_child(Rc::new(RefCell::new(c)));
    x.borrow_mut().set_left_child(Rc::new(RefCell::new(a)));
    y.borrow_mut().set_parent(x.clone());
    x.borrow_mut().set_right_child(y);
    x.borrow_mut().set_parent(sentinel.clone());
    let mut actual_tree = Tree::construct(x, sentinel);

    // Build the expected Tree after performing a left rotate
    //
    //                4
    //               /  \
    //              2    5
    //            /  \
    //           1     3
    //
    let sentinel = Rc::new(RefCell::new(Node::new_sentinel()));
    let mut a = Node::new(1);
    a.set_left_child(sentinel.clone());
    a.set_right_child(sentinel.clone());
    let mut b = Node::new(3);
    b.set_left_child(sentinel.clone());
    b.set_right_child(sentinel.clone());
    let mut c = Node::new(5);
    c.set_left_child(sentinel.clone());
    c.set_right_child(sentinel.clone());
    let y = Rc::new(RefCell::new(Node::new(4)));
    let x = Rc::new(RefCell::new(Node::new(2)));
    x.borrow_mut().set_left_child(Rc::new(RefCell::new(a)));
    x.borrow_mut().set_right_child(Rc::new(RefCell::new(b)));
    x.borrow_mut().set_parent(y.clone());
    y.borrow_mut().set_left_child(x);
    y.borrow_mut().set_right_child(Rc::new(RefCell::new(c)));
    y.borrow_mut().set_parent(sentinel.clone());
    let expected_tree = Tree::construct(y, sentinel);

    actual_tree.left_rotate(actual_tree.root.clone());

    assert_eq!(actual_tree, expected_tree);
}

#[test]
fn test_right_rotate() {
    // Build the expected Tree after performing a left rotate
    //
    //                4
    //               /  \
    //              2    5
    //            /  \
    //           1     3
    //
    let sentinel = Rc::new(RefCell::new(Node::new_sentinel()));
    let mut a = Node::new(1);
    a.set_left_child(sentinel.clone());
    a.set_right_child(sentinel.clone());
    let mut b = Node::new(3);
    b.set_left_child(sentinel.clone());
    b.set_right_child(sentinel.clone());
    let mut c = Node::new(5);
    c.set_left_child(sentinel.clone());
    c.set_right_child(sentinel.clone());
    let y = Rc::new(RefCell::new(Node::new(4)));
    let x = Rc::new(RefCell::new(Node::new(2)));
    x.borrow_mut().set_left_child(Rc::new(RefCell::new(a)));
    x.borrow_mut().set_right_child(Rc::new(RefCell::new(b)));
    x.borrow_mut().set_parent(y.clone());
    y.borrow_mut().set_left_child(x);
    y.borrow_mut().set_right_child(Rc::new(RefCell::new(c)));
    y.borrow_mut().set_parent(sentinel.clone());
    let mut actual_tree = Tree::construct(y, sentinel);

    // Build an expected Tree that looks like this:
    //
    //               2
    //             /   \
    //            1      4
    //                  /  \
    //                 3    5
    //
    let sentinel = Rc::new(RefCell::new(Node::new_sentinel()));
    let mut a = Node::new(1);
    a.set_left_child(sentinel.clone());
    a.set_right_child(sentinel.clone());
    let mut b = Node::new(3);
    b.set_left_child(sentinel.clone());
    b.set_right_child(sentinel.clone());
    let mut c = Node::new(5);
    c.set_left_child(sentinel.clone());
    c.set_right_child(sentinel.clone());
    let y = Rc::new(RefCell::new(Node::new(4)));
    let x = Rc::new(RefCell::new(Node::new(2)));
    c.set_parent(y.clone());
    b.set_parent(y.clone());
    a.set_parent(x.clone());
    y.borrow_mut().set_left_child(Rc::new(RefCell::new(b)));
    y.borrow_mut().set_right_child(Rc::new(RefCell::new(c)));
    x.borrow_mut().set_left_child(Rc::new(RefCell::new(a)));
    y.borrow_mut().set_parent(x.clone());
    x.borrow_mut().set_right_child(y);
    x.borrow_mut().set_parent(sentinel.clone());
    let expected_tree = Tree::construct(x, sentinel);

    actual_tree.right_rotate(actual_tree.root.clone());

    assert_eq!(actual_tree, expected_tree);
}

// // TODO tests
// // Tests properies of the red black tree. (these are the properties of the red black tree that are abstracted away)
// // Test over multiple interesting types (u32, u64, i32, i64, string)
// // Test over multiple deleteing types
// // Test all public methods
//
fn is_red<T>(node: Rc<RefCell<Node<T>>>) -> bool {
    node.borrow().color == NodeColor::Red
}

fn is_black<T>(node: Rc<RefCell<Node<T>>>) -> bool
where
    T: Default,
{
    !is_red(node)
}

fn check_red_node_property<T>(node: Rc<RefCell<Node<T>>>) -> bool
where
    T: Default,
{
    match node.borrow().is_nil() {
        false => {
            if is_red(node.clone()) {
                // Ensure children are black
                is_black(node.borrow().left().clone()) && is_black(node.borrow().right().clone())
            } else {
                // Recursively check children
                check_red_node_property(node.borrow().left().clone())
                    && check_red_node_property(node.borrow().right().clone())
            }
        }
        true => true, // Nil nodes are black
    }
}

fn count_black_nodes<T>(node: Rc<RefCell<Node<T>>>) -> Vec<i32>
where
    T: Default,
{
    match node.borrow().is_nil() {
        false => {
            let left_counts = count_black_nodes(node.borrow().left().clone());
            let right_counts = count_black_nodes(node.borrow().right().clone());

            // Combine and adjust for current node
            let mut counts = vec![];
            for count in left_counts.into_iter().chain(right_counts.into_iter()) {
                counts.push(
                    count
                        + if node.borrow().color == NodeColor::Black {
                            1
                        } else {
                            0
                        },
                );
            }
            counts
        }
        true => vec![1], // Nil nodes are black, count as one black node
    }
}

fn assert_red_black_tree_properties<T>(tree: &Tree<T>)
where
    T: Default,
{
    if tree.root.borrow().is_nil() {
        panic!("Assertions on empty red-black trees cause a panic for your own sake")
    }
    // Root property
    assert_eq!(tree.root.borrow().color, NodeColor::Black);

    // Red node property
    assert!(check_red_node_property(tree.root.clone()));

    // Black height property
    let black_node_counts = count_black_nodes(tree.root.clone());
    // Ensure all paths have the same number of black nodes
    assert_eq!(
        black_node_counts.iter().min(),
        black_node_counts.iter().max()
    );
}

#[test]
fn test_insert_maintains_properties() {
    let mut tree = Tree::new(10);
    assert_red_black_tree_properties(&tree);
    tree.insert(5);
    assert_red_black_tree_properties(&tree);
    tree.insert(15);
    assert_red_black_tree_properties(&tree);
    tree.insert(2);
    assert_red_black_tree_properties(&tree);
    tree.insert(7);
    assert_red_black_tree_properties(&tree);
    tree.insert(12);
    assert_red_black_tree_properties(&tree);
    tree.insert(17);
    assert_red_black_tree_properties(&tree);
}

#[test]
fn test_insert_types() {
    let mut string_tree = Tree::new("Red".to_string());
    string_tree.insert("Black".to_string());
    string_tree.insert("Tree".to_string());

    assert_red_black_tree_properties(&string_tree);

    let mut i64_tree = Tree::new(-10_i64);
    i64_tree.insert(0_i64);
    i64_tree.insert(100_i64);

    assert_red_black_tree_properties(&i64_tree);

    let mut f64_tree = Tree::new(-100.03_f64);
    f64_tree.insert(0.0_f64);
    f64_tree.insert(75_f64);

    assert_red_black_tree_properties(&f64_tree);
}

#[test]
fn test_delete_maintains_properties() {
    let mut tree = Tree::new(10);
    tree.delete(10);
    assert!(tree.root.borrow().is_nil());

    tree.insert(5);
    tree.insert(15);
    tree.insert(2);
    tree.insert(7);
    tree.insert(12);
    tree.insert(17);

    tree.delete(17);
    assert_red_black_tree_properties(&tree);
    tree.delete(7);
    assert_red_black_tree_properties(&tree);
    tree.delete(2);
    assert_red_black_tree_properties(&tree);
}

proptest! {
    #[test]
    fn test_minimum_empirical(a in 0u32..u32::MAX, b in 0u32..u32::MAX, c in 0u32..u32::MAX) {
        let mut tree = Tree::empty();
        println!("a: {}", a);
        tree.insert(a);
        println!("b: {}", b);
        tree.insert(b);
        println!("c: {}", c);
        tree.insert(c);
        let actual_min = tree.minimum();

        let expected_min = min(min(a, b), c);

        println!("Actual_min={:?}, expected_min={}", actual_min, expected_min);
        prop_assert_eq!(actual_min, Some(expected_min));
    }
}
