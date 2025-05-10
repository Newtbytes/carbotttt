use lorax::{Block, Cursor, Operation, Value, Var};

use crate::ops::*;

fn binop_pat(op: &Operation) -> (&str, &[Value], Option<Var>) {
    (op.name.as_ref(), &op.operands[..], op.result)
}

pub fn lower_binop(cursor: &mut Cursor<Operation>) {
    let op = cursor.get().unwrap();

    match binop_pat(op) {
        (name, [src], Some(dst)) => {
            let new_op = match name {
                "arith.neg" => neg(dst),
                "arith.complement" => todo!(),
                _ => return (),
            };

            cursor.push_behind(mov(*src, dst));
            cursor.replace(new_op);
        }
        _ => (),
    }
}
