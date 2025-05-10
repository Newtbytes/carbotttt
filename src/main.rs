use clap::Parser;

use error::CompilerError;
use lorax::{Block, Operation};
use parser::ast;

//mod asm;
//mod driver;
mod error;
mod parser;
mod src;

fn main() -> () {
    let mut block = lorax::Block {
        operations: Vec::new(),
    };

    let stmt = ast::Stmt::Return(ast::Expr::Unary(
        ast::UnaryOp::Negate,
        Box::new(ast::Expr::Unary(
            ast::UnaryOp::Negate,
            Box::new(ast::Expr::Constant(1)),
        )),
    ));

    parser::lower_stmt(&mut block, &stmt);
    let mut cursor: lorax::Cursor<Operation> = (&mut block).into();
    x86::lower_binop(&mut cursor);

    println!("{}", block);

    // match driver::run_compiler(driver::Cli::parse()) {
    //     Err(e) => Err(e),
    //     _ => Ok(()),
    // }
}
