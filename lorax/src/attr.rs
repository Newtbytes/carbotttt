use std::collections::HashMap;

#[derive(Debug)]
pub enum Attribute {
    Int(u32),
}

pub type AttributeMap = HashMap<String, Attribute>;
