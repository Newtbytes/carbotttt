use crate::{Block, Operation, RewriteRule, RewriteRuleSet, Value, walk_blocks};

pub struct RewritingCtx<'a> {
    block: &'a mut Block,
    op_idx: usize,
}

impl<'a> RewritingCtx<'a> {
    pub fn new(block: &'a mut Block) -> Self {
        Self { block, op_idx: 0 }
    }

    pub fn replace_block(&mut self, block: &'a mut Block) {
        self.block = block;

        // reset the op_idx to 0
        self.op_idx = 0;
    }

    fn advance(&mut self) {
        if self.op_idx < self.block.len() {
            self.op_idx += 1;
        }
    }

    pub fn get(&self) -> &Operation {
        self.block
            .body
            .get(self.op_idx)
            .expect("op_idx should always point to an existing operation")
    }

    pub fn get_mut(&mut self) -> &mut Operation {
        self.block
            .body
            .get_mut(self.op_idx)
            .expect("op_idx should always point to an existing operation")
    }

    pub fn operands(&self) -> &[Value] {
        self.get().operands.as_slice()
    }

    pub fn name(&self) -> &str {
        &self.get().name
    }

    pub fn prev(&self) -> Option<&Operation> {
        self.block.body.get(self.op_idx - 1)
    }

    pub fn result(&self) -> &Option<Value> {
        &self.get().result
    }

    pub fn insert_behind(&mut self, op: Operation) -> Option<Value> {
        self.block.insert(self.op_idx, op);

        // adjust index so we continue to point to the correct op
        self.advance();

        self.prev().map(|op| op.result).flatten()
    }

    pub fn insert_ahead(&mut self, op: Operation) {
        self.block.insert(self.op_idx + 1, op);
    }

    pub fn replace(&mut self, new: Operation) {
        *(self.get_mut()) = new;
    }

    pub fn done(&self) -> bool {
        self.op_idx >= self.block.len()
    }

    pub fn release(self) {}
}

pub fn rewrite_blocks<'a, 'b>(block: &'a mut Block, pass: RewriteRuleSet<RewritingCtx<'b>>)
where
    Block: 'a,
    'a: 'b,
{
    for bl in walk_blocks(block) {
        let mut ctx = RewritingCtx::new(bl);

        while !ctx.done() {
            pass.apply(&mut ctx);
            ctx.advance();
        }

        ctx.release();
    }
}
