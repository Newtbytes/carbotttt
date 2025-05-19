use std::{fmt::Display, sync::atomic};

use crate::pool::{Pool, Ptr};

#[derive(Debug, Clone, Copy)]
pub struct Value {
    id: usize,
    pub(crate) def: Option<Ptr>,
}

impl Value {
    pub fn new(ptr: Option<Ptr>) -> Self {
        static TMP_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

        Self {
            id: TMP_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
            def: ptr,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.id)
    }
}

pub type OpResult = Option<Value>;

#[derive(Debug)]
pub struct Operation {
    pub name: &'static str,
    pub operands: Vec<Value>,
    pub blocks: Vec<Block>,
    pub result: OpResult,
    pub attr: Option<u32>,
}

impl Operation {
    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_result(&self) -> Value {
        self.result
            .expect("this should be called on an op with at least one result")
    }

    pub fn get_mut_result(&mut self) -> &mut Value {
        self.result
            .as_mut()
            .expect("this should be called on an op with at least one result")
    }

    pub fn walk_blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    pub fn walk_blocks_mut(&mut self) -> impl Iterator<Item = &mut Block> {
        self.blocks.iter_mut()
    }
}

#[macro_export]
macro_rules! def_op {
    // Block-only operation (no operands, no result)
    ($dl:ident . $name:ident ($field:ident : Block)) => {
        pub fn $name($field: Block) -> Operation {
            Operation {
                name: stringify!($dl . $name),
                operands: Vec::new(),
                blocks: vec![$field],
                result: None,
                attr: None,
            }
        }
    };

    // Operation with operands, optional result
    ($dl:ident . $name:ident ( $($field:ident : $ty:ty),* $(,)? ) $(-> $ret:ident)? ) => {
        pub fn $name($($field: $ty),*) -> Operation {
            Operation {
                name: stringify!($dl . $name),
                operands: vec![$($field.into()),*],
                blocks: Vec::new(),
                result: def_op!(@ret $( $ret )?),
                attr: None
            }
        }
    };

    // Operation with one attribute
    ($dl:ident . $name:ident (  ) { value: $ty:ty }) => {
        pub fn $name(value: $ty) -> Operation {
            Operation {
                name: stringify!($dl . $name),
                operands: Vec::new(),
                blocks: Vec::new(),
                result: Some(Value::new(None)),
                attr: Some(value),
            }
        }
    };

    // Result handling
    (@ret) => { Some(Value::new(None)) };
    (@ret None) => { None };
    (@ret Value) => { Some(Value::new()) };
    (@ret $ret:ident) => { Some(($ret).into()) };
}

fn fmt_delimited_list<I>(list: &mut I, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    I: Iterator,
    I::Item: Display,
{
    if let Some(item) = list.next() {
        write!(f, "{}", item)?;
    }

    for item in list {
        write!(f, ", {}", item)?;
    }

    Ok(())
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(var) = self.result {
            write!(f, "{} := {} ", var, self.name)?;
        } else {
            write!(f, "{} ", self.name)?;
        }

        fmt_delimited_list(&mut self.operands.iter(), f)?;

        if let Some(val) = self.attr {
            write!(f, "{{ value: {val} }}")?;
        }

        if !self.blocks.is_empty() {
            write!(f, "\n")?;
        }

        for block in &self.blocks {
            write!(f, "{}", block)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Block {
    pub(crate) id: usize,
    pub pool: Pool<Operation>,
}

impl Block {
    pub(crate) fn unique_id() -> usize {
        static BLOCK_ID_COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
        BLOCK_ID_COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
    }

    pub fn new() -> Self {
        Self {
            id: Self::unique_id(),
            pool: Pool::new(),
        }
    }

    pub fn get(&self, ptr: Ptr) -> &Operation {
        self.pool.deref(ptr)
    }

    pub fn get_mut(&mut self, ptr: Ptr) -> &mut Operation {
        self.pool.deref_mut(ptr)
    }

    pub fn walk_ops(&self) -> impl Iterator<Item = &Operation> {
        self.pool.iter()
    }

    pub fn walk_ops_mut(&mut self) -> impl Iterator<Item = &mut Operation> {
        self.pool.iter_mut()
    }

    pub fn push(&mut self, op: Operation) -> Ptr {
        self.pool.alloc(op)
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    /// Traverse value definitions in each operation's operands
    /// to create a linear sequence of operations.
    pub fn linearize(&self) -> Vec<Ptr> {
        let mut linearized = Vec::new();

        for (id, op) in self.walk_ops().enumerate() {
            for operand in &op.operands {
                if let Some(def) = operand.def {
                    if linearized.iter().any(|x| *x == def) {
                        continue;
                    }
                    linearized.push(def);
                }
            }

            linearized.push(id.into());
        }

        linearized
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ".bb{}:", self.id)?;

        for ptr in self.linearize() {
            writeln!(f, "    {}", self.get(ptr))?;
        }

        Ok(())
    }
}

// this is incorect, but for now it will do
pub fn walk_blocks<'a>(block: &'a mut Block) -> Box<dyn Iterator<Item = &'a mut Block> + 'a> {
    let mut blocks = Vec::new();

    for op in block.walk_ops_mut() {
        blocks.extend(op.walk_blocks_mut());
    }

    Box::new(blocks.into_iter())
}
