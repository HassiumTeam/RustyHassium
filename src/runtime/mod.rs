use std::collections::HashMap;

use crate::parser::{BinOpType, UnaryOpType};

pub enum VMInstruction {
    BinOp { op: BinOpType },
    Break,
    Call { arg_count: u32 },
    Continue,
    Import,
    Invoke { arg_count: u32 },
    Iter,
    IterFull,
    IterNext,
    Jump { to: u32 },
    JumpIfFalse { to: u32 },
    JumpIfTrue { to: u32 },
    LoadAttrib { attrib: String },
    LoadId { id: String },
    LoadNumber { value: f64 },
    LoadString { value: String },
    LoadSubscript,
    Pop,
    Raise,
    Return,
    SelfRef,
    StoreAttrib { attrib: String },
    StoreId { id: String },
    StoreSubscript,
    Super { arg_count: u32 },
    UnaryOp { op: UnaryOpType },
}

pub struct CodeObj {
    pub instructions: Vec<VMInstruction>,
    pub labels: HashMap<u32, u32>,
}
