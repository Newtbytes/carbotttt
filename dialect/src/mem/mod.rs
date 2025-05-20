use lorax::{Operation, Value, def_op};

def_op! {
    mem.alloca(size: Value) -> Value
}

def_op! {
    mem.dealloca(size: Value) -> None
}
