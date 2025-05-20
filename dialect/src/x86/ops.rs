use lorax::{Operation, Value, def_op};

def_op! {
    x86.mov(src: Value, dst: Value) -> dst
}

def_op! {
    x86.svar(offset: Value) -> Value
}

def_op! {
    x86.subq(num: Value, reg: Value) -> reg
}

def_op! {
    x86.pushq(num: Value) -> None
}

def_op! {
    x86.popq(num: Value) -> None
}

def_op! {
    x86.neg(src: Value) -> src
}

def_op! {
    x86.not(src: Value) -> src
}

def_op! {
    x86.ret() -> None
}
