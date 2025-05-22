// Global allocation context for all IR nodes in Lorax

use crate::ir::{Block, Operation};

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

#[derive(Default)]
pub struct Ctx {
    pub blocks: Vec<Block>,
    pub operations: Vec<Operation>,
}

impl Ctx {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc_block(&mut self, block: Block) -> BlockId {
        let id = BlockId(self.blocks.len());
        self.blocks.push(block);
        id
    }

    pub fn get_block(&self, id: BlockId) -> &Block {
        &self.blocks[id.0]
    }

    pub fn get_block_mut(&mut self, id: BlockId) -> &mut Block {
        &mut self.blocks[id.0]
    }

    pub fn alloc_operation(&mut self, op: Operation) -> OperationId {
        let id = OperationId(self.operations.len());
        self.operations.push(op);
        id
    }

    pub fn get_operation(&self, id: OperationId) -> &Operation {
        &self.operations[id.0]
    }

    pub fn get_operation_mut(&mut self, id: OperationId) -> &mut Operation {
        &mut self.operations[id.0]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperationId(pub usize);
