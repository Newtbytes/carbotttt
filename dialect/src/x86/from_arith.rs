use lorax::{RewriteRule, RewritingCtx};

use super::ops::*;

pub struct LowerBinop;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerBinop {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands(), ctx.result()) {
            (name, &[src], Some(dst)) => {
                let op = ctx.alloc_op(mov(src, dst));
                let val = op.get_result();
                ctx.replace(match name {
                    "arith.negate" => neg(val),
                    "arith.complement" => not(val),
                    _ => return (),
                });
            }
            _ => (),
        }
    }
}
