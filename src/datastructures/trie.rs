#![allow(dead_code)]
use std::collections::HashMap;

struct TrieNode<T> {
    children: HashMap<char, TrieNode<T>>,
    val: Option<T>,
}

impl<T> TrieNode<T> {
    fn new() -> TrieNode<T> {
        TrieNode {
            children: HashMap::new(),
            val: None,
        }
    }
}

struct TrieTree<T> {
    root: TrieNode<T>,
}

impl<T> TrieTree<T> {
    fn new() -> TrieTree<T> {
        TrieTree {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, s: &str, val: T) {
        let mut node = &mut self.root;
        for c in s.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new())
        }
        node.val = Some(val)
    }

    fn get(&self, s: &str) -> Option<&T> {
        let mut node = &self.root;
        for c in s.chars() {
            if node.children.contains_key(&c) {
                node = node.children.get(&c).unwrap();
            } else {
                return None;
            }
        }
        node.val.as_ref()
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for TrieTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for TrieNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[val: {:?}] -> \n children:", self.val);
        write!(f, "{:?} \n", self.children.keys());
        for (_, val) in self.children.iter() {
            write!(f, "{val} \n");
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie() {
        let mut tree: TrieTree<i32> = TrieTree::new();
        tree.insert("lzz", 123);
        tree.insert("lzq", 345);
        assert_eq!(tree.get("lzz"), Some(&123));
        assert_eq!(tree.get("lzq"), Some(&345));
        println!("{}", tree);
    }
}
