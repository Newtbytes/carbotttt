use lorax::{RewriteRuleSet, RewritingCtx};

mod emit;
mod from_arith;
mod from_func;
mod from_mem;
mod ops;
mod state;

pub fn rules<'ctx>() -> RewriteRuleSet<RewritingCtx<'ctx>> {
    RewriteRuleSet::new()
        .add_rule(from_arith::LowerBinop)
        .add_rule(from_func::LowerFunc)
        .add_rule(from_mem::LowerMem)
}
