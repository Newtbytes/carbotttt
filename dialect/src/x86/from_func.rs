use lorax::{LinkedList, RewriteRule, RewritingCtx};

use super::{
    ops::*,
    state::{ax, rbp, rsp},
};

pub struct LowerFunc;
impl<'block> RewriteRule<RewritingCtx<'block>> for LowerFunc {
    fn apply(&self, ctx: &mut RewritingCtx<'block>) {
        match (ctx.name(), ctx.operands()) {
            ("func.func", _) => {
                let f = ctx.get_mut();

                if let Some(block) = f.blocks.first_mut() {
                    if let Some(head) = *block.head() {
                        let rbp = block.insert_behind(head, rbp());
                        let rbp = block.get(rbp).get_result();

                        let rsp = block.insert_behind(head, rsp());
                        let rsp = block.get(rsp).get_result();

                        block.insert_behind(head, pushq(rbp));
                        block.insert_behind(head, mov(rsp, rbp));
                    } else {
                        let rbp = block.push(rbp());
                        let rbp = block.get(rbp).get_result();

                        let rsp = block.push(rsp());
                        let rsp = block.get(rsp).get_result();

                        block.push(pushq(rbp));
                        block.push(mov(rsp, rbp));
                    }
                }
            }

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
