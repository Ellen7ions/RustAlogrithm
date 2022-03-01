use std::fmt::{self, Debug, Display};

struct BinarySearchNode<T: Ord + Debug> {
    value: Option<T>,
    left: Option<Box<BinarySearchNode<T>>>,
    right: Option<Box<BinarySearchNode<T>>>,
}

impl<T: Ord + Debug> BinarySearchNode<T> {
    fn new(v: T) -> BinarySearchNode<T> {
        BinarySearchNode {
            value: Some(v),
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, v: T) {
        if let Some(ref val) = self.value {
            if *val < v {
                if let Some(ref mut r) = self.right {
                    r.insert(v);
                } else {
                    self.right = Some(Box::new(BinarySearchNode::new(v)));
                }
            } else {
                if let Some(ref mut l) = self.left {
                    l.insert(v);
                } else {
                    self.left = Some(Box::new(BinarySearchNode::new(v)));
                }
            }
        } else {
            self.value = Some(v)
        }
    }
}

impl<T: Ord + Debug> Debug for BinarySearchNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "root->{:?} -> \n", self.value);
        write!(f, "=================\n");
        write!(f, "left->\n{:?}\n", self.left);
        write!(f, "=================\n");
        write!(f, "right->\n{:?}\n", self.right);
        write!(f, "-----------------\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bst() {
        let mut bst = BinarySearchNode::new(8);
        bst.insert(4);
        bst.insert(9);
        bst.insert(6);
        bst.insert(10);
        println!("{:?}", bst);
    }
}
