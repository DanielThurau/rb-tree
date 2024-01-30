use std::cell::{Ref, RefCell};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::node::{Node, NodeColor};

/// red-black properties
/// --------------------
///
/// 1. Every node is either red or black
/// 2. The root is black
/// 3. Every leaf (None) is black
/// 4. If a node is red, then both its children are black
/// 5. For each node, all simple paths from the node to
///    descendant leaves contain the same number of black
///    nodes.

struct Tree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord> Tree<T> where Node<T>: PartialEq {
    pub fn new(key: T) -> Tree<T> {
        Tree {
            root: Some(Rc::new(RefCell::new(Node::new(key)))),
        }
    }

    fn new_from_node(node: Node<T>) -> Tree<T> {
        Tree {
            root: Some(Rc::new(RefCell::new(node)))
        }
    }

    pub fn insert(&mut self, key: T) {
        let mut z = Node::new(key);
        let mut x = self.root.clone();
        let mut y = None;

        while x.is_some() {
            y = x.clone();
            if z.key < x.as_ref().unwrap().borrow().key {
                let x_tmp = x.as_ref().unwrap().borrow().left.clone();
                x = x_tmp
            } else {
                let x_tmp = x.as_ref().unwrap().borrow().right.clone();
                x = x_tmp;

            }
        }
        z.parent = y.clone();
        // Z is now Reference counted for
        let z = Rc::new(RefCell::new(z));

        if y.is_none() {
            self.root = Some(z.clone());
        } else if z.borrow().key < y.as_ref().unwrap().borrow().key {
            y.as_mut().unwrap().borrow_mut().left = Some(z.clone());
        } else {
            y.as_mut().unwrap().borrow_mut().right = Some(z.clone());
        }
        z.borrow_mut().left = None;
        z.borrow_mut().right = None;
        z.borrow_mut().color = NodeColor::Red;
        self.insert_fix_up(z);
    }

    fn insert_fix_up(&mut self, mut z: Rc<RefCell<Node<T>>>) {
        if z.borrow().parent.is_none() {
            panic!("You violated an invariant. Z's parent cannot be none.");
        }
        while z.borrow().parent.is_some() && z.borrow().parent.as_ref().unwrap().borrow().color == NodeColor::Red {
            if z.borrow().parent == z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left {
                let y = z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().right.clone().unwrap();
                // Case 1
                if y.borrow().color == NodeColor::Red {
                    println!("Case 1");
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                    let z_tmp = z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                    z = z_tmp;
                } else {
                    if z == z.borrow().parent.as_ref().unwrap().borrow().right.as_ref().unwrap().clone() {
                        println!("Case 2");
                        let z_tmp = z.borrow().parent.as_ref().unwrap().clone();
                        z = z_tmp;
                        self.left_rotate(z.clone());
                    }
                    // Case 3
                    println!("Case 3");
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                    self.right_rotate(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone());
                }
            } else {
                let y = z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().borrow().left.clone().unwrap();
                // Case 4
                if y.borrow().color == NodeColor::Red {
                    println!("Case 4");
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                    let z_tmp = z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                    z = z_tmp;
                } else {
                    // Case 5
                    if z == z.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap().clone() {
                        println!("Case 5");
                        let z_tmp = z.borrow().parent.as_ref().unwrap().clone();
                        z = z_tmp;
                        self.right_rotate(z.clone());
                    }
                    // Case 6
                    println!("Case 6");
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut().parent.as_mut().unwrap().borrow_mut().parent.as_mut().unwrap().borrow_mut().color = NodeColor::Red;
                    self.right_rotate(z.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone());
                }
            }

        }
        self.root.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
    }

    pub fn delete(&mut self, _key: T) {}

    fn left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {

        if x.borrow().right.is_some() {
            let y = x.borrow_mut().right.take().unwrap();
            x.borrow_mut().set_right_child(y.borrow().left.clone());
            if y.borrow().left.is_some() {
                y.borrow_mut().left.as_mut().unwrap().borrow_mut().parent = Some(x.clone());
            }
            y.borrow_mut().set_parent(x.borrow().parent.clone());
            if x.borrow().parent.is_none() {
                self.root = Some(y.clone());
            } else if Some(x.clone()) == x.borrow().parent.as_ref().unwrap().borrow().left {
                x.borrow_mut().parent.as_mut().unwrap().borrow_mut().left = Some(y.clone());
            } else {
                x.borrow_mut().parent.as_mut().unwrap().borrow_mut().right = Some(y.clone());
            }
            y.borrow_mut().left = Some(x.clone());
            x.borrow_mut().parent = Some(y);

        } else {
            panic!("I don't have the implementation for this yet.")

        }
    }

    fn right_rotate(&mut self, y: Rc<RefCell<Node<T>>>) {
        if y.borrow().left.is_some() {
            let x = y.borrow_mut().left.take().unwrap();
            y.borrow_mut().set_left_child(x.borrow().right.clone());
            if x.borrow().right.is_some() {
                x.borrow_mut().right.as_mut().unwrap().borrow_mut().parent = Some(y.clone());
            }
            x.borrow_mut().set_parent(y.borrow().parent.clone());
            if y.borrow().parent.is_none() {
                self.root = Some(x.clone());
            } else if Some(y.clone()) == y.borrow().parent.as_ref().unwrap().borrow().left {
                y.borrow_mut().parent.as_mut().unwrap().borrow_mut().right = Some(x.clone());
            } else {
                y.borrow_mut().parent.as_mut().unwrap().borrow_mut().left = Some(x.clone());
            }
            x.borrow_mut().right = Some(y.clone());
            y.borrow_mut().parent = Some(x);
        } else {
            panic!("I Don't have the implementation for this yet.")
        }
    }

    pub fn search(&self, _key: T) -> bool {
        todo!()
    }

    pub fn minimum(&self) -> T {
        todo!()
    }

    pub fn successor(&self) -> T {
        todo!()
    }

    pub fn predecessor(&self) -> T {
        todo!()
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

#[test]
fn test_rotate() {
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);

    let mut y = Node::new(4);
    let mut x = Node::new(2);
    y.set_left_child(Some(Rc::new(RefCell::new(b))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(y))));

    println!("{:?}", x);
    println!("Root: {:?}", x.key);

    let mut t = Tree::new_from_node(x);

    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);

    t.left_rotate(t.root.as_ref().unwrap().clone());

    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);

    t.right_rotate(t.root.as_ref().unwrap().clone());
    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);
}


#[test]
fn test_insert() {
    let mut t = Tree::new(2);
    println!("{:?}", t);

    t.insert(1);
    println!("{:?}", t);

    t.insert(4);
    println!("{:?}", t);


    t.insert(3);
    println!("{:?}", t);
    t.insert(5);
    println!("{:?}", t);
}