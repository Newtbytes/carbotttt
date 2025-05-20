use lorax::{Operation, Value, def_op};

def_op! {
    arith.negate(val: Value)
}

def_op! {
    arith.complement(val: Value)
}

def_op! {
    arith.constant() {
        value: u32
    }
}
