use lorax::{RewriteRule, RewritingCtx};

use super::{ops::*, state::ax};

pub struct LowerFunc;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerFunc {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands()) {
            ("func.ret", &[val]) => {
                let ax = ctx.insert_behind(ax());
                ctx.insert_behind(mov(val, ax.unwrap()));
                ctx.replace(ret());
            }
            _ => (),
        }
    }
}
