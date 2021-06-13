mod tree;
use crate::avl::tree::{AvlNode, AvlTree};
use crate::memtable::Memtable;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeMap<T: Ord, U>
where
    T: Clone + Sync + Send,
    U: Clone + Sync + Send,
{
    root: AvlTree<T, U>,
}
impl<T: Ord + Clone + Sync + Send, U: Clone + Sync + Send> AvlTreeMap<T, U> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
impl<T: Ord + Clone + Sync + Send, U: Clone + Sync + Send> Memtable<T, U> for AvlTreeMap<T, U> {
    fn insert(&mut self, key: T, value: U) {
        if let Some(node) = &mut self.root {
            node.insert(key, value);
        } else {
            self.root = Some(Box::new(AvlNode::new(key, value)));
        }
    }
    fn delete(&mut self, key: &T) {
        if let Some(node) = &mut self.root {
            self.root = node.clone().delete(key)
        } else {
            self.root = None
        }
    }
    fn search(&self, key: &T) -> Option<&U> {
        self.root.as_ref().map_or(None, |node| node.search(key))
    }
}

impl<'a, T: 'a + Ord + Clone + Sync + Send, U: Clone + Sync + Send> AvlTreeMap<T, U> {
    fn iter(&'a self) -> AvlTreeSetIter<'a, T, U> {
        AvlTreeSetIter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
struct AvlTreeSetIter<'a, T: Ord, U>
where
    T: Clone + Sync + Send,
    U: Clone + Sync + Send,
{
    prev_nodes: Vec<&'a AvlNode<T, U>>,
    current_tree: &'a AvlTree<T, U>,
}

impl<'a, T: 'a + Ord + Clone + Sync + Send, U: Clone + Sync + Send> Iterator for AvlTreeSetIter<'a, T, U> {
    type Item = (&'a T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match *self.current_tree {
                None => match self.prev_nodes.pop() {
                    None => return None,
                    Some(ref prev_node) => {
                        self.current_tree = &prev_node.right;
                        return Some((&prev_node.key, &prev_node.value));
                    }
                },
                Some(ref current_node) => {
                    if current_node.left.is_some() {
                        self.prev_nodes.push(&current_node);
                        self.current_tree = &current_node.left;
                        continue;
                    }
                    if current_node.right.is_some() {
                        self.current_tree = &current_node.right;
                        return Some((&current_node.key, &current_node.value));
                    }
                    self.current_tree = &None;
                    return Some((&current_node.key, &current_node.value));
                }
            }
        }
    }
}
impl<T: Ord + Clone + Sync + Send, U: Clone + Sync + Send> FromIterator<(T, U)> for AvlTreeMap<T, U> {
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let mut set = Self::new();

        for (key, value) in iter {
            set.insert(key, value);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use crate::avl::AvlTreeMap;
    use crate::memtable::Memtable;

    #[test]
    fn iter() {
        let mut map = AvlTreeMap::new();

        for i in (1..4 as usize).rev() {
            map.insert(i, i + 1);
        }

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), Some((&3, &4)));
        assert_eq!(iter.next(), None);
    }
}
