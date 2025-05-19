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
                ctx.replace(subq(size, rsp.unwrap()));
            }

            // function epilogue
            ("x86.ret", _) => {
                let rbp = ctx.insert_behind(rbp());
                let rsp = ctx.insert_behind(rsp());
                ctx.insert_behind(mov(rbp.unwrap(), rsp.unwrap()));
                ctx.insert_behind(popq(rbp.unwrap()));
            }
            _ => (),
        }
    }
}
