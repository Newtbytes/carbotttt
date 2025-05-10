use lorax::{Operation, Region, Value};

pub fn func(region: Region) -> Operation {
    Operation {
        name: "fn".to_owned(),
        operands: Vec::new(),
        regions: vec![region],
        result: None,
    }
}

pub fn ret<V: Into<Value>>(val: V) -> Operation {
    Operation {
        name: "ret".to_owned(),
        operands: vec![val.into()],
        regions: Vec::new(),
        result: None,
    }
}
