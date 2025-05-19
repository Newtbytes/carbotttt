// Lower AST to IR

use lorax::{Block, Value, builtin};

use super::ast;

use dialect::{
    arith,
    func::{func, ret},
};

fn lower_expr(block: &mut Block, expr: &ast::Expr) -> Value {
    let op = match expr {
        ast::Expr::Unary(unary_op, expr) => match unary_op {
            ast::UnaryOp::Complement => todo!(),
            ast::UnaryOp::Negate => arith::negate(lower_expr(block, expr)),
        },

        ast::Expr::Constant(val) => builtin::constant(*val),
    };

    let ptr = block.push(op);
    block.get(ptr).get_result()
}

pub fn lower_stmt(block: &mut Block, stmt: &ast::Stmt) {
    let op = match stmt {
        ast::Stmt::Return(expr) => ret(lower_expr(block, expr)),
    };

    block.push(op);
}

pub fn lower_program(program: &ast::Program) -> Block {
    let mut region = Block::new();

    match &program.body {
        ast::Decl::Function(_, stmt) => {
            let mut block = Block::new();

            lower_stmt(&mut block, stmt);
            region.push(func(block));
        }
    };

    region
}
