pub struct LinkedNode<T: Sized> {
    contained: T,
    pub prev: Option<*mut LinkedNode<T>>,
    pub next: Option<*mut LinkedNode<T>>,
}

impl<T> LinkedNode<T> {
    fn new(value: T) -> LinkedNode<T> {
        LinkedNode {
            contained: value,
            prev: None,
            next: None,
        }
    }

    fn get(&self) -> &T {
        &self.contained
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.contained
    }

    fn insert_ahead(&mut self, value: T) {
        let node: *mut LinkedNode<T> = &mut LinkedNode::new(value);

        if let Some(next) = self.next {
            unsafe {
                (*next).prev = Some(node);
                (*node).next = Some(next);
            }
        }
    }
}
