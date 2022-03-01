use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct Node<T: Debug> {
    pub data: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    pub fn new(v: Option<T>) -> Node<T> {
        Node {
            data: v,
            next: None,
        }
    }

    pub fn append(&mut self, node: Node<T>) {
        match self.next {
            Some(ref mut d) => d.append(node),
            None => self.next = Some(Box::new(node)),
        }
    }
}

pub struct LinkedList<T: Debug> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: Some(Box::new(Node::new(None))),
        }
    }

    pub fn push_back(&mut self, node: Node<T>) {
        match self.head {
            Some(ref mut cur_node) => {
                cur_node.append(node);
            }
            None => {
                self.head = Some(Box::new(node));
            }
        }
    }
}

impl<T: Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut p = &self.head;
        // write!(f, "{:?} ", p.data);
        while let Some(ref d) = p {
            write!(f, "{:?} ", d.data)?;
            p = &(d.next);
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(Node::new(Some(1)));
        list.push_back(Node::new(Some(2)));
        list.push_back(Node::new(Some(3)));
        println!("{}", list);
    }
}
