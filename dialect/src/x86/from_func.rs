use lorax::{RewriteRule, RewritingCtx};

use super::{ops::*, state::ax};

pub struct LowerFunc;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerFunc {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands()) {
            ("func.ret", &[val]) => {
                let v0 = ctx.insert_behind(ax());
                let v0 = ctx.result_of(v0);
                let _ = ctx.insert_behind(mov(val, v0));

                ctx.replace(ret());
            }
            _ => (),
        }
    }
}
