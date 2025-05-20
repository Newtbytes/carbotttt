use crate::{
    Block, Operation, RewriteRule, RewriteRuleSet, Value, link::LinkedList, pool::Ptr, walk_blocks,
};

pub struct RewritingCtx<'a> {
    block: &'a mut Block,
    op: Ptr,
}

impl<'a> RewritingCtx<'a> {
    pub fn new(block: &'a mut Block, op: Ptr) -> Self {
        Self { block, op }
    }

    pub fn from_start(block: &'a mut Block) -> Self {
        Self::new(block, Ptr::new(0))
    }

    /// Allocate an operation in the pool, filling in the op's result with a def
    pub fn alloc_op(&mut self, op: Operation) -> &Operation {
        let ptr = self.block.pool.alloc(op);

        if let Some(val) = &mut self.deref_mut(ptr).result {
            if let None = val.def {
                val.def = Some(ptr);
            }
        }

        self.deref(ptr)
    }

    fn advance(&mut self) {
        if self.op < self.block.pool.len().into() {
            self.op.idx += 1;
        }
    }

    pub fn get(&self) -> &Operation {
        self.block.pool.deref(self.op)
    }

    pub fn get_mut(&mut self) -> &mut Operation {
        self.block.pool.deref_mut(self.op)
    }

    pub fn deref(&self, ptr: Ptr) -> &Operation {
        self.block.pool.deref(ptr)
    }

    pub fn deref_mut(&mut self, ptr: Ptr) -> &mut Operation {
        self.block.pool.deref_mut(ptr)
    }

    pub fn result_of(&mut self, ptr: Ptr) -> Value {
        self.deref(ptr).get_result()
    }

    pub fn insert_behind(&mut self, op: Operation) -> Ptr {
        self.block.insert_behind(self.op, op)
    }

    pub fn operands<'b>(&'a self) -> &'b [Value]
    where
        'a: 'b,
    {
        &self.get().operands.as_slice()
    }

    pub fn name(&self) -> &'static str {
        self.get().name
    }

    pub fn result(&self) -> Option<Value> {
        self.get().result
    }

    pub fn replace(&mut self, new: Operation) {
        *(self.get_mut()) = new;
    }

    pub fn done(&self) -> bool {
        self.op.idx >= self.block.pool.len()
    }

    pub fn release(self) {}
}

pub fn rewrite_ops<'a, 'b>(block: &'a mut Block, pass: RewriteRuleSet<RewritingCtx<'b>>)
where
    Block: 'a,
    'a: 'b,
{
    for bl in walk_blocks(block) {
        let mut ctx = RewritingCtx::from_start(bl);

        while !ctx.done() {
            pass.apply(&mut ctx);
            ctx.advance();
        }

        ctx.release();
    }
}
