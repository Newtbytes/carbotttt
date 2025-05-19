use crate::{
    Block, Operation, RewriteRule, RewriteRuleSet, Value,
    pool::{Pool, Ptr},
    walk_blocks,
};

pub struct RewritingCtx<'a> {
    pool: &'a mut Pool<Operation>,
    op: Ptr,
}

impl<'a> RewritingCtx<'a> {
    pub fn new(pool: &'a mut Pool<Operation>, op: Ptr) -> Self {
        Self { pool, op }
    }

    pub fn from_block(block: &'a mut Block) -> Self {
        Self {
            pool: &mut block.pool,
            op: Ptr::new(0),
        }
    }

    /// Allocate an operation in the pool, filling in the op's result with a def
    pub fn alloc_op(&mut self, op: Operation) -> &Operation {
        let ptr = self.pool.alloc(op);

        if let Some(val) = &mut self.deref_mut(ptr).result {
            if let None = val.def {
                val.def = Some(ptr);
            }
        }

        self.deref(ptr)
    }

    fn advance(&mut self) {
        if self.op < self.pool.len().into() {
            self.op.idx += 1;
        }
    }

    pub fn get<'op>(&'a self) -> &'op Operation
    where
        'a: 'op,
    {
        self.pool.deref(self.op)
    }

    pub fn get_mut(&mut self) -> &mut Operation {
        self.pool.deref_mut(self.op)
    }

    pub fn deref(&self, ptr: Ptr) -> &Operation {
        self.pool.deref(ptr)
    }

    pub fn deref_mut(&mut self, ptr: Ptr) -> &mut Operation {
        self.pool.deref_mut(ptr)
    }

    pub fn operands<'b>(&'a self) -> &'b [Value]
    where
        'a: 'b,
    {
        &self.get().operands.as_slice()
    }

    pub fn name<'b>(&'a self) -> &'b str
    where
        'a: 'b,
    {
        self.get().name
    }

    pub fn result(&self) -> Option<Value> {
        self.get().result
    }

    pub fn replace(&mut self, new: Operation) {
        *(self.get_mut()) = new;
    }

    pub fn done(&self) -> bool {
        self.op.idx >= self.pool.len()
    }

    pub fn release(self) {}
}

pub fn rewrite_ops<'a, 'b>(block: &'a mut Block, pass: RewriteRuleSet<RewritingCtx<'b>>)
where
    Block: 'a,
    'a: 'b,
{
    for bl in walk_blocks(block) {
        let mut ctx = RewritingCtx::from_block(bl);

        while !ctx.done() {
            pass.apply(&mut ctx);
            ctx.advance();
        }

        ctx.release();
    }
}
