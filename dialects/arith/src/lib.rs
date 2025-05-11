use lorax::{Operation, Value, Var};

pub fn negate<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "arith.neg".to_owned(),
        operands: vec![val.into()],
        blocks: Vec::new(),
        result: Some(Var::new()),
    }
}
