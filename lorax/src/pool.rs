use std::ops::Deref;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Ptr {
    pub(crate) idx: usize,
}

impl Ptr {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

impl From<usize> for Ptr {
    fn from(idx: usize) -> Self {
        Self { idx }
    }
}

#[derive(Debug)]
pub struct Pool<T> {
    objs: Vec<T>,
}

impl<'a, T> Pool<T> {
    pub fn new() -> Self {
        Pool { objs: Vec::new() }
    }

    pub fn reserve(&mut self, count: usize) {
        self.objs.reserve(count);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Pool {
            objs: Vec::with_capacity(capacity),
        }
    }

    pub fn alloc(&mut self, obj: T) -> Ptr {
        self.objs.push(obj);

        Ptr {
            idx: self.objs.len() - 1,
        }
    }

    pub fn deref(&self, ptr: Ptr) -> &T {
        self.objs.get(ptr.idx).expect("Deref of dangling ptr")
    }

    pub fn deref_mut(&mut self, ptr: Ptr) -> &mut T {
        self.objs
            .get_mut(ptr.idx)
            .expect("Mut deref of dangling ptr")
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.objs.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.objs.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.objs.len()
    }
}
