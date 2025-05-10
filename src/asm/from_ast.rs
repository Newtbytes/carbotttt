use lorax::{Rewritable, rewrite, rewrite_rule};

use super::nodes as asm;
use crate::parser::ast;

rewrite_rule! {
    ast::Expr => asm::Operand {
        ast::Expr::Constant(value) => asm::Operand::Imm(*value),
        ast::Expr::Unary(unary_op, expr) => todo!("Unary expr lowering")
    }
}

rewrite_rule! {
    ast::Stmt => Vec<asm::Instruction> {
        ast::Stmt::Return(expr) => vec![
            asm::Instruction::Mov {
                src: rewrite(expr),
                dst: asm::Operand::Register,
            },
            asm::Instruction::Ret,
        ],
    }
}

rewrite_rule! {
    ast::Decl => asm::Decl {
        ast::Decl::Function(name, stmt) => asm::Decl::Function {
            name: name.to_string(),
            body: rewrite(stmt.as_ref()),
        },
    }
}

rewrite_rule! {
    ast::Program => asm::Program {
        ast::Program { body } => asm::Program {
            body: rewrite(body),
        },
    }
}
