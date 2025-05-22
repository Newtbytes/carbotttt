use crate::{
    Block, Operation, RewriteRule, RewriteRuleSet, Value,
    ctx::{Ctx, OperationId, Ptr},
    link::LinkedList,
};

pub struct RewritingCtx<'a> {
    ctx: &'a mut Ctx,
    block: &'a mut Block,
    op: Ptr,
}

impl<'a> RewritingCtx<'a> {
    pub fn new(ctx: &'a mut Ctx, block: &'a mut Block, op: Ptr) -> Self {
        Self { ctx, block, op }
    }

    pub fn from_start(ctx: &'a mut Ctx, block: &'a mut Block) -> Self {
        Self::new(ctx, block, Ptr::new(0))
    }

    /// Allocate an operation in the global context, returning its Ptr
    pub fn alloc_op(&mut self, op: Operation) -> Ptr {
        let op_id = self.ctx.alloc_operation(op);
        let ptr = Ptr::new(op_id.0);
        self.block.push(ptr);
        if let Some(val) = &mut self.ctx.get_operation_mut(op_id).result {
            if val.def.is_none() {
                val.def = Some(ptr);
            }
        }
        ptr
    }

    fn advance(&mut self) {
        if self.op.idx + 1 < self.block.len() {
            self.op.idx += 1;
        }
    }

    pub fn get(&self) -> &Operation {
        let ptr = self.block.ops[self.op.idx];
        self.ctx.get_operation(OperationId(ptr.idx))
    }

    pub fn get_mut(&mut self) -> &mut Operation {
        let ptr = self.block.ops[self.op.idx];
        self.ctx.get_operation_mut(OperationId(ptr.idx))
    }

    pub fn deref(&self, ptr: Ptr) -> &Operation {
        self.ctx.get_operation(OperationId(ptr.idx))
    }

    pub fn deref_mut(&mut self, ptr: Ptr) -> &mut Operation {
        self.ctx.get_operation_mut(OperationId(ptr.idx))
    }

    pub fn insert_behind(&mut self, op: Operation) -> Ptr {
        self.block.insert_behind(self.ctx, self.op, op)
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
        let ptr = self.block.ops[self.op.idx];
        *self.ctx.get_operation_mut(OperationId(ptr.idx)) = new;
    }

    pub fn done(&self) -> bool {
        self.op.idx >= self.block.len()
    }

    pub fn release(self) {}
}

pub fn rewrite_ops<'a, 'b>(
    ctx: &'a mut Ctx,
    block: &'a mut Block,
    pass: RewriteRuleSet<RewritingCtx<'b>>,
) where
    Block: 'a,
    'a: 'b,
{
    // Only rewrite the top-level block for now
    let mut rctx = RewritingCtx::from_start(ctx, block);
    while !rctx.done() {
        pass.apply(&mut rctx);
        rctx.advance();
    }
    rctx.release();
}
