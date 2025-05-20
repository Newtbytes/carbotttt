use crate::{Pool, pool::Ptr};

pub trait LinkedNode {
    fn ahead(&self) -> Option<Ptr>;
    fn behind(&self) -> Option<Ptr>;
    fn ahead_mut(&mut self) -> &mut Option<Ptr>;
    fn behind_mut(&mut self) -> &mut Option<Ptr>;
}

pub struct LinkedListIter<'a, T: LinkedNode> {
    pool: &'a Pool<T>,
    current: Option<Ptr>,
}

impl<'a, T: LinkedNode> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let curr_ptr = self.current?;
        let node = self.pool.deref(curr_ptr);
        self.current = node.ahead();
        Some(node)
    }
}

pub trait LinkedList<T: LinkedNode> {
    fn head(&self) -> &Option<Ptr>;
    fn tail(&self) -> &Option<Ptr>;
    fn head_mut(&mut self) -> &mut Option<Ptr>;
    fn tail_mut(&mut self) -> &mut Option<Ptr>;

    fn pool(&self) -> &Pool<T>;
    fn pool_mut(&mut self) -> &mut Pool<T>;

    fn insert_behind(&mut self, root: Ptr, inserted: T) -> Ptr {
        let pool = self.pool_mut();
        let inserted = pool.alloc(inserted);

        if let Some(behind) = *pool.deref_mut(root).behind_mut() {
            // link up inserted node between the old behind node and the root
            *pool.deref_mut(inserted).behind_mut() = Some(behind);
            *pool.deref_mut(inserted).ahead_mut() = Some(root);

            // the old behind node now points to inserted
            *pool.deref_mut(behind).ahead_mut() = Some(inserted);

            // point the root's behind ptr to the inserted node
            *pool.deref_mut(root).behind_mut() = Some(inserted);
        }

        inserted
    }

    fn push(&mut self, node: T) -> Ptr {
        let node = self.pool_mut().alloc(node);

        if let Some(tail_ptr) = *self.tail_mut() {
            let tail = self.pool_mut().deref_mut(tail_ptr);
            *tail.ahead_mut() = Some(node);

            let node = self.pool_mut().deref_mut(node);
            *node.behind_mut() = Some(tail_ptr)
        }

        *self.tail_mut() = Some(node);

        if self.head().is_none() {
            *self.head_mut() = Some(node);
        }

        node
    }

    fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            pool: self.pool(),
            current: *self.head(),
        }
    }
}
