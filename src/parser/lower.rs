// Lower AST to IR

use lorax::{Block, Constant, Region, Value};

use super::ast;

use arith;
use func;

fn lower_expr(block: &mut Block, expr: &ast::Expr) -> Value {
    let op = match expr {
        ast::Expr::Unary(unary_op, expr) => match unary_op {
            ast::UnaryOp::Complement => todo!(),
            ast::UnaryOp::Negate => arith::negate(lower_expr(block, expr)),
        },

        ast::Expr::Constant(val) => return Constant { val: *val }.into(),
    };

    block.push(op).get_result().into()
}

pub fn lower_stmt(block: &mut Block, stmt: &ast::Stmt) {
    let op = match stmt {
        ast::Stmt::Return(expr) => func::ret(lower_expr(block, expr)),
    };

    block.push(op);
}

pub fn lower_program(program: &ast::Program) -> Region {
    let mut region = Region::new();

    match &program.body {
        ast::Decl::Function(_, stmt) => {
            let mut block = Block::new();
            lower_stmt(&mut block, &stmt);
            region.push(block);
        }
    };

    return region;
}
