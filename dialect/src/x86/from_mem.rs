use lorax::{RewriteRule, RewritingCtx};

use crate::mem::alloca;

use super::{
    ops::*,
    state::{rbp, rsp},
};

pub struct LowerMem;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerMem {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands()) {
            ("mem.alloca", &[size]) => {
                let rsp = ctx.insert_behind(rsp());
                let rsp = ctx.result_of(rsp);
                ctx.replace(subq(size, rsp));
            }

            // function epilogue
            ("x86.ret", _) => {
                let rbp = ctx.insert_behind(rbp());
                let rsp = ctx.insert_behind(rsp());

                let rbp = ctx.result_of(rbp);
                let rsp = ctx.result_of(rsp);

                ctx.insert_behind(mov(rbp, rsp));
                ctx.insert_behind(popq(rbp));
            }
            _ => (),
        }
    }
}
