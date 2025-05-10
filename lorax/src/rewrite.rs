use std::ops::{Deref, DerefMut};

pub trait RewriteRule<T> {
    fn apply(&self, node: &mut T);
}

/// A collection of rewrite rules, applied in a specific order.
pub struct RewriteRuleSet<T> {
    rules: Vec<Box<dyn RewriteRule<T>>>,
}

impl<T> RewriteRule<T> for RewriteRuleSet<T> {
    fn apply(&self, node: &mut T) {
        for rule in &self.rules {
            rule.apply(node);
        }
    }
}

impl<T> RewriteRule<Vec<T>> for RewriteRuleSet<T> {
    fn apply(&self, nodes: &mut Vec<T>) {
        for node in nodes {
            self.apply(node);
        }
    }
}

#[derive(Debug)]
pub struct Cursor<'block, T> {
    pub(crate) nodes: &'block mut Vec<T>,
    pub(crate) idx: usize,
}

impl<'block, T> Cursor<'block, T> {
    pub fn new<C>(nodes: &'block mut C) -> Self
    where
        C: DerefMut<Target = Vec<T>>,
    {
        Cursor { nodes, idx: 0 }
    }

    pub fn get(&self) -> Option<&T> {
        self.nodes.get(self.idx)
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.nodes.get_mut(self.idx)
    }

    pub fn push_behind(&mut self, op: T) {
        // insert node behind the current idx,
        // clamping the idx to 0 if needed
        let idx = if self.idx > 0 { self.idx - 1 } else { 0 };

        self.nodes.insert(idx, op);

        // the node we were pointing to has changed by 1
        // if nodes is empty, it now has one element
        if !self.nodes.is_empty() {
            self.idx += 1;
        }
    }

    pub fn push_ahead(&mut self, op: T) {
        self.nodes.insert(self.idx + 1, op);
    }

    pub fn advance(&mut self) {
        self.idx += 1;
    }

    pub fn replace(&mut self, op: T) {
        *(self.get_mut().unwrap()) = op;
    }
}
