mod tree;
use crate::avl::tree::{AvlNode, AvlTree};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeMap<K: Ord, V>
where
    K: Clone + Sync + Send,
    V: Clone + Sync + Send,
{
    root: AvlTree<K, V>,
}
impl<K: Ord + Clone + Sync + Send, V: Clone + Sync + Send> AvlTreeMap<K, V> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
impl<K: Ord + Clone + Sync + Send, V: Clone + Sync + Send> AvlTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(node) = &mut self.root {
            node.insert(key, value);
        } else {
            self.root = Some(Box::new(AvlNode::new(key, value)));
        }
    }
    fn delete(&mut self, key: &K) {
        if let Some(node) = &mut self.root {
            self.root = node.clone().delete(key)
        } else {
            self.root = None
        }
    }
    pub fn search(&self, key: &K) -> Option<&V> {
        self.root.as_ref().map_or(None, |node| node.search(key))
    }
}

impl<K: Ord + Clone + Sync + Send, V: Clone + Sync + Send> AvlTreeMap<K, V> {
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            prev_nodes: Vec::new(),
            current_tree: &self.root,
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, K: Ord, V>
where
    K: Clone + Sync + Send,
    V: Clone + Sync + Send,
{
    prev_nodes: Vec<&'a AvlNode<K, V>>,
    current_tree: &'a AvlTree<K, V>,
}

impl<'a, K: 'a + Ord + Clone + Sync + Send, V: Clone + Sync + Send> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
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
impl<K: Ord + Clone + Sync + Send, V: Clone + Sync + Send> FromIterator<(K, V)>
    for AvlTreeMap<K, V>
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
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
