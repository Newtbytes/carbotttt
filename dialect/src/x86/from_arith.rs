use lorax::{RewriteRule, RewritingCtx, Value};

use super::ops::*;

pub struct LowerBinop;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerBinop {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands(), ctx.result()) {
            (name, &[src], Some(dst)) => {
                let ptr = ctx.insert_behind(mov(src, dst));
                let ptr = ctx.deref(ptr).get_result();

                ctx.replace(match name {
                    "arith.negate" => neg(ptr),
                    "arith.complement" => not(ptr),
                    _ => return (),
                });
            }
            _ => (),
        }
    }
}
