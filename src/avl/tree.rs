use std::cmp::{max, Ordering};
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlNode<T: Ord, U>
where
    T: Clone,
    U: Clone,
{
    pub key: T,
    pub value: U,
    height: usize,
    pub left: AvlTree<T, U>,
    pub right: AvlTree<T, U>,
}
pub type AvlTree<T, U> = Option<Box<AvlNode<T, U>>>;
impl<T: Ord + Clone, U: Clone> AvlNode<T, U> {
    pub fn new(key: T, value: U) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
            height: 1,
        }
    }
    pub fn insert(&mut self, key: T, value: U) {
        match self.key.cmp(&key) {
            Ordering::Less => {
                if let Some(node) = &mut self.right {
                    node.insert(key, value);
                    node.update_height();
                } else {
                    self.right = Option::from(Box::new(Self::new(key, value)));
                    self.update_height();
                }
            }
            Ordering::Greater => {
                if let Some(node) = &mut self.left {
                    node.insert(key, value);
                    node.update_height();
                } else {
                    self.left = Option::from(Box::new(Self::new(key, value)));
                    self.update_height();
                }
            }
            Ordering::Equal => self.value = value,
        }
        self.rebalance();
    }
    pub fn delete(mut self, key: &T) -> AvlTree<T, U> {
        match self.key.cmp(key) {
            Ordering::Less => {
                if let Some(right) = self.right {
                    self.right = right.delete(key);
                    self.rebalance();
                    return Option::from(Box::new(self));
                }
            }
            Ordering::Greater => {
                if let Some(left) = self.left {
                    self.left = left.delete(key);
                    self.rebalance();
                    return Option::from(Box::new(self));
                }
            }
            Ordering::Equal => return self.delete_root(),
        }
        return Option::from(Box::new(self));
    }
    fn delete_root(self) -> AvlTree<T, U> {
        match (self.left, self.right) {
            (None, None) => None,
            (Some(l), None) => Option::from(l),
            (None, Some(r)) => Option::from(r),
            (Some(l), Some(r)) => Option::from(Box::new(r.combine(l))),
        }
    }
    fn combine(self, left: Box<AvlNode<T, U>>) -> AvlNode<T, U> {
        let (rest, max_key, max_value) = left.delete_max();
        let mut new_root = Self::new(max_key, max_value);
        new_root.left = rest;
        new_root.right = Option::from(Box::new(self));
        new_root.rebalance();
        new_root
    }
    fn delete_max(mut self) -> (AvlTree<T, U>, T, U) {
        match self.right {
            Some(right) => {
                let (rest, key, value) = right.delete_max();
                self.right = rest;
                self.rebalance();
                (Option::from(Box::new(self)), key, value)
            }
            None => (self.left, self.key, self.value),
        }
    }
    fn left_height(&self) -> usize {
        self.left.as_ref().map_or(0, |l| l.height)
    }
    fn right_height(&self) -> usize {
        self.right.as_ref().map_or(0, |r| r.height)
    }
    fn update_height(&mut self) {
        self.height = 1 + max(self.left_height(), self.right_height())
    }
    fn balance_factor(&self) -> i8 {
        let left_height = self.left_height();
        let right_height = self.right_height();
        left_height as i8 - right_height as i8
    }
    fn rotate_right(&mut self) {
        let mut cloned_self = self.clone();
        let mut left = self.left.as_ref().unwrap().clone();
        cloned_self.left = left.clone().right;
        cloned_self.update_height();
        left.right = Option::from(Box::new(cloned_self));
        *self = *left;
    }
    fn rotate_left(&mut self) {
        let mut cloned_self = self.clone();
        let mut right = self.right.as_ref().unwrap().clone();
        cloned_self.right = right.clone().left;
        cloned_self.update_height();
        right.left = Option::from(Box::new(cloned_self));
        *self = *right;
    }

    fn rebalance(&mut self) {
        let balance_factor = self.balance_factor();
        if balance_factor < -1 {
            let right_node = self.right.as_mut().unwrap();
            if right_node.balance_factor() > 0 {
                right_node.rotate_right();
            }
            self.rotate_left();
        } else if balance_factor > 1 {
            let left_node = self.left.as_mut().unwrap();
            if left_node.balance_factor() < 0 {
                left_node.rotate_left();
            }
            self.rotate_right();
        }
        self.update_height()
    }

    pub fn search(&self, key: &T) -> Option<&U> {
        match self.key.cmp(key) {
            Ordering::Less => self.right.as_ref().map_or(None, |node| node.search(key)),
            Ordering::Greater => self.left.as_ref().map_or(None, |node| node.search(key)),
            Ordering::Equal => Option::from(&self.value),
        }
    }
}
impl<T: Ord + Display + Clone, U: Clone> AvlNode<T, U> {
    fn print(&self) {
        self.print_tree(0);
    }
    fn print_tree(&self, depth: usize) {
        self.right
            .as_ref()
            .into_iter()
            .for_each(|n| n.print_tree(depth + 1));
        println!("{}+{}", " ".repeat(depth), self.key);
        self.left
            .as_ref()
            .into_iter()
            .for_each(|n| n.print_tree(depth + 1));
    }
}

#[cfg(test)]
mod tests {
    use crate::avl::AvlNode;

    #[test]
    fn insert() {
        let mut node = AvlNode::new(1, 1);
        for i in (1..3 as usize).rev() {
            node.insert(i, i + 1);
        }
        assert_eq!(
            node,
            AvlNode {
                key: 1,
                value: 2,
                height: 2,
                left: None,
                right: Some(Box::new(AvlNode {
                    key: 2,
                    value: 3,
                    height: 1,
                    left: None,
                    right: None
                })),
            }
        );
    }
    #[test]
    fn rebalance() {
        let mut node = AvlNode::new(1, 1);

        for i in (2..10 as usize).rev() {
            node.insert(i, i);
        }
        assert_eq!(
            node,
            AvlNode {
                key: 6,
                value: 6,
                height: 4,
                left: Some(Box::new(AvlNode {
                    key: 4,
                    value: 4,
                    height: 3,
                    left: Some(Box::new(AvlNode {
                        key: 2,
                        value: 2,
                        height: 2,
                        left: Some(Box::new(AvlNode {
                            key: 1,
                            value: 1,
                            height: 1,
                            left: None,
                            right: None
                        })),
                        right: Some(Box::new(AvlNode {
                            key: 3,
                            value: 3,
                            height: 1,
                            left: None,
                            right: None
                        }))
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 5,
                        value: 5,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
                right: Some(Box::new(AvlNode {
                    key: 8,
                    value: 8,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        key: 7,
                        value: 7,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 9,
                        value: 9,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
            }
        );
    }

    #[test]
    fn delete() {
        let mut node = AvlNode::new(1, 1);
        for i in (2..10 as usize).rev() {
            node.insert(i, i);
        }
        let mut new_node = *node.clone().delete(&2).unwrap();
        new_node = *new_node.delete(&5).unwrap();
        assert_eq!(
            new_node,
            AvlNode {
                key: 6,
                value: 6,
                height: 3,
                left: Some(Box::new(AvlNode {
                    key: 3,
                    value: 3,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        key: 1,
                        value: 1,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 4,
                        value: 4,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
                right: Some(Box::new(AvlNode {
                    key: 8,
                    value: 8,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        key: 7,
                        value: 7,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 9,
                        value: 9,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
            }
        );

        new_node = *node.clone().delete(&8).unwrap();
        new_node = *new_node.delete(&9).unwrap();
        assert_eq!(
            new_node,
            AvlNode {
                key: 4,
                value: 4,
                height: 3,
                left: Some(Box::new(AvlNode {
                    key: 2,
                    value: 2,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        key: 1,
                        value: 1,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 3,
                        value: 3,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
                right: Some(Box::new(AvlNode {
                    key: 6,
                    value: 6,
                    height: 2,
                    left: Some(Box::new(AvlNode {
                        key: 5,
                        value: 5,
                        height: 1,
                        left: None,
                        right: None
                    })),
                    right: Some(Box::new(AvlNode {
                        key: 7,
                        value: 7,
                        height: 1,
                        left: None,
                        right: None
                    }))
                })),
            }
        );
    }
    fn search() {
        let mut node = AvlNode::new(1, 1);
        for i in (2..10 as usize).rev() {
            node.insert(i, i);
        }
        assert_eq!(node.search(&1), Some(&1));
        assert_eq!(node.search(&0), None)
    }
}
